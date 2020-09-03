//  Copyright (c) 2020 Christopher Taylor
//
//  SPDX-License-Identifier: BSL-1.0
//  Distributed under the Boost Software License, Version 1.0. (See accompanying
//  file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
// find out system targets: rustc --target=i686-pc-windows-msvc --print target-cpus
//
// compile flag : rustc src\main.rs -C target-cpu=<pick one from target-cpus> -C target-feature=+sse3,+avx
//
use std::fs::read_to_string;
use std::path::Path;
use std::vec;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::clone;
use std::mem;

#[derive(Copy, Clone)]
pub struct FloatType {
    value : f64,
}

#[derive(Copy, Clone)]
pub struct IntegerType {
    value : i64,
}

pub struct StringType {
    value : String,
}

impl FloatType {

    // snagged this from stackoverflow
    // TODO need to identify the link
    //
    pub fn integer_decode(val: f64) -> Vec<u64> {
        let bits: u64 = unsafe { mem::transmute(val) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };
    
        exponent -= 1023 + 52;
        vec![mantissa as u64, exponent as u64, sign as u64]
    }
}

impl Hash for FloatType {
    #[inline]
    fn hash<H>(&self, mut state: &mut H) where H: Hasher {        
        let fbytearray = FloatType::integer_decode(self.value);
        fbytearray.hash(&mut state);
        state.finish();
    }    
}

impl Hash for IntegerType {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.value.hash(state);
    }    
}

impl Hash for StringType {
    #[inline]
    fn hash<H>(&self, mut state: &mut H) where H: Hasher {
        self.value.as_str().hash(&mut state);
        state.finish();
    }    
}

impl Hash for DataTypes {
    #[inline]
    fn hash<H>(&self, mut state: &mut H) where H: Hasher {
        match self {
            DataTypes::FloatType(FloatType{value}) => {
                let fbytearray = FloatType::integer_decode(*value);
                fbytearray.hash(&mut state);
                state.finish();
            },
            DataTypes::IntegerType(IntegerType{value}) => {
                value.hash(state);
            },
            DataTypes::StringType(StringType{value}) => {
                value.as_str().hash(&mut state);
                state.finish();
            },
        }
    }    
}

#[derive(Clone)]
pub enum DataTypes {
    FloatType(FloatType),
    IntegerType(IntegerType),
    StringType(StringType),
}

impl Clone for StringType {
    #[inline]
    fn clone(&self) -> StringType {
        StringType{ value : self.value.to_string() }
    }
}

impl PartialEq for FloatType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for FloatType {}

impl PartialEq for IntegerType {    
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IntegerType {}

impl PartialEq for StringType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for StringType {}

impl PartialEq for DataTypes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let operands = (self, other);

        match operands {
            (DataTypes::FloatType(FloatType{value : lvalue}), DataTypes::FloatType(FloatType{value : rvalue})) => {
                lvalue == rvalue
            },
            (DataTypes::IntegerType(IntegerType{value : lvalue}), DataTypes::IntegerType(IntegerType{value : rvalue})) => {
                lvalue == rvalue
            },
            (DataTypes::StringType(StringType{value : lvalue}), DataTypes::StringType(StringType{value : rvalue})) => {
                lvalue == rvalue
            },
            _ => false
        }
    }
}

impl Eq for DataTypes {}

impl DataTypes {
    pub fn println(&self) {
        match self {
            DataTypes::FloatType(FloatType{value}) => { println!("{}", value); }
            DataTypes::IntegerType(IntegerType{value})=> { println!("{}", value); }
            DataTypes::StringType(StringType{value})=> { println!("{}", value); }
        }
    }

    pub fn print(&self) {
        match self {
            DataTypes::FloatType(FloatType{value}) => { print!("{}", value); }
            DataTypes::IntegerType(IntegerType{value})=> { print!("{}", value); }
            DataTypes::StringType(StringType{value})=> { print!("{}", value); }
        }
    }

    pub fn fvalue(&self) -> f64 {
        match self {
            DataTypes::FloatType(FloatType{value}) => { *value }
            DataTypes::IntegerType(IntegerType{value}) => { *value as f64 }
            _ => { f64::NAN }
        }
    }

    pub fn ivalue(&self) -> i64 {
        match self {
            DataTypes::FloatType(FloatType{value}) => { *value as i64 }
            DataTypes::IntegerType(IntegerType{value}) => { *value }
            DataTypes::StringType(StringType{value})=> {
                let mut s = DefaultHasher::new();
                value.as_str().hash(&mut s);
                s.finish() as i64
            }
        }
    }

    pub fn svalue(&self) -> String {
        match self {
            DataTypes::FloatType(FloatType{value}) => { value.to_string() }
            DataTypes::IntegerType(IntegerType{value}) => { value.to_string() }
            DataTypes::StringType(StringType{value}) => { String::from(value) }
            //_ => { String::from("NOT A STRING") }
        }
    }
}

trait CalculateColumn {
    fn column(column : & Vec<DataTypes>, column_name : &str) -> DataFrame;
}

impl CalculateColumn for FloatType {   
    fn column(columns : &Vec<DataTypes>, column_name : &str) -> DataFrame {
        let cpy : Vec<DataTypes> = columns.iter().map(|i| {
            DataTypes::FloatType(FloatType{ value : i.fvalue() })
        }).collect();

        return DataFrame{ labels : std::vec![column_name.to_string(),], columns : std::vec![cpy,] };
    }
}

impl CalculateColumn for IntegerType {
    fn column(columns : &Vec<DataTypes>, column_name : &str) -> DataFrame {
        let cpy : Vec<DataTypes> = columns.iter().map(|i| {
            DataTypes::IntegerType(IntegerType{ value : i.ivalue() })
        }).collect();

        return DataFrame{ labels : std::vec![column_name.to_string(),], columns : std::vec![cpy,] };
    }
}

impl CalculateColumn for StringType {        
    fn column(columns : &Vec<DataTypes>, column_name : &str) -> DataFrame {
        let cpy : Vec<DataTypes> = columns.iter().map(|i| {
            DataTypes::FloatType(FloatType{ value : i.fvalue() })
        }).collect();

        return DataFrame{ labels : std::vec![column_name.to_string(),], columns : std::vec![cpy,] };
    }
}

trait CalculateSum {
    fn sum(column : & Vec<DataTypes>) -> DataTypes;
}

impl CalculateSum for FloatType {   
    fn sum(columns : &Vec<DataTypes>) -> DataTypes {
        let fvalue : f64 = columns.iter().fold(0.0, |sum, i| sum + match i {
            DataTypes::FloatType(FloatType{value}) => { *value }
            _ => 0.0
        });

        return DataTypes::FloatType(FloatType{value : fvalue});
    }
}

impl CalculateSum for IntegerType {
    fn sum(columns : &Vec<DataTypes>) -> DataTypes {
        let return_value : i64 = columns.iter().fold(0, |sum, i| sum + match i {
            DataTypes::IntegerType(IntegerType{value}) => { *value } 
            _ => 0
        });

        return DataTypes::IntegerType(IntegerType{value : return_value});
    }
}

impl CalculateSum for StringType {        
    fn sum(_ : &Vec<DataTypes>) -> DataTypes {
        return DataTypes::StringType(StringType{value : String::from("NAN")})
    }
}

trait CalculateMean {
    fn mean(column : & Vec<DataTypes>) -> DataTypes;
}

impl CalculateMean for FloatType {   
    fn mean(columns : &Vec<DataTypes>) -> DataTypes {
        let fvalue : f64 = columns.iter().fold(0.0, |sum, i| sum + match i {
            DataTypes::FloatType(FloatType{value}) => { *value }
            _ => 0.0
        });

        return DataTypes::FloatType(FloatType{value : fvalue / columns.len() as f64});
    }
}

impl CalculateMean for IntegerType {
    fn mean(columns : &Vec<DataTypes>) -> DataTypes {
        let return_value : i64 = columns.iter().fold(0, |sum, i| sum + match i {
            DataTypes::IntegerType(IntegerType{value}) => { *value } 
            _ => 0
        });

        return DataTypes::IntegerType(IntegerType{value : return_value / columns.len() as i64});
    }
}

impl CalculateMean for StringType {        
    fn mean(_ : &Vec<DataTypes>) -> DataTypes {
        return DataTypes::StringType(StringType{value : String::from("NAN")})
    }
}

trait CalculateStdDev {
    fn stddev(column : & Vec<DataTypes>) -> DataTypes;
}

impl CalculateStdDev for FloatType {   
    fn stddev(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : f64 = FloatType::mean(&columns).fvalue();

        let return_value : f64 = columns.iter().fold(0.0, |sum, i| sum + match i {
            DataTypes::FloatType(FloatType{value}) => { (*value - mean_value).powf(2.0) } 
            _ => 0.0
        });

        let stddev_value : f64 = (return_value / (columns.len() as f64)).sqrt();

        return DataTypes::FloatType(FloatType{value : stddev_value});        
    }
}

impl CalculateStdDev for IntegerType {
    fn stddev(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : i64 = IntegerType::mean(&columns).ivalue();

        let return_value : i64 = columns.iter().fold(0, |sum, i| sum + match i {
            DataTypes::IntegerType(IntegerType{value}) => { (*value - mean_value).pow(2) } 
            _ => 0
        });

        let stddev_value : i64 = (( return_value / ( columns.len() as i64)) as f64).sqrt() as i64;

        return DataTypes::IntegerType(IntegerType{value : stddev_value});
    }
}

impl CalculateStdDev for StringType {        
    fn stddev(_ : &Vec<DataTypes>) -> DataTypes {
        return DataTypes::StringType(StringType{value : String::from("NAN")})
    }
}

trait CalculatePStdDev {
    fn pstddev(column : & Vec<DataTypes>) -> DataTypes;
}

impl CalculatePStdDev for FloatType {   
    fn pstddev(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : f64 = FloatType::mean(&columns).fvalue();
        let denom : f64 = columns.len() as f64;

        let return_value : f64 = columns.iter().fold(0.0, |sum, i| sum + match i {
            DataTypes::FloatType(FloatType{value}) => { (*value - mean_value).powf(2.0) / denom } 
            _ => 0.0
        });

        return DataTypes::FloatType(FloatType{value : return_value.sqrt()});        
    }
}

impl CalculatePStdDev for IntegerType {
    fn pstddev(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : i64 = IntegerType::mean(&columns).ivalue();
        let denom : i64 = columns.len() as i64;

        let return_value : i64 = columns.iter().fold(0, |sum, i| sum + match i {
            DataTypes::IntegerType(IntegerType{value}) => { (*value - mean_value).pow(2) / denom } 
            _ => 0
        });

        return DataTypes::IntegerType(IntegerType{value : (return_value as f64).sqrt() as i64});
    }
}

impl CalculatePStdDev for StringType {        
    fn pstddev(_ : &Vec<DataTypes>) -> DataTypes {
        return DataTypes::StringType(StringType{value : String::from("NAN")})
    }
}

trait CalculateVariance {
    fn variance(column : & Vec<DataTypes>) -> DataTypes;
}

impl CalculateVariance for FloatType {   
    fn variance(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : f64 = FloatType::mean(&columns).fvalue();
        let denom : f64 = columns.len() as f64;

        let return_value : f64 = columns.iter().fold(0.0, |sum, i| sum + match i {
            DataTypes::FloatType(FloatType{value}) => { (*value - mean_value).powf(2.0) } 
            _ => 0.0
        });

        return DataTypes::FloatType(FloatType{value : return_value / denom});        
    }
}

impl CalculateVariance for IntegerType {
    fn variance(columns : &Vec<DataTypes>) -> DataTypes {
        let mean_value : i64 = IntegerType::mean(&columns).ivalue();
        let denom : i64 = columns.len() as i64;

        let return_value : i64 = columns.iter().fold(0, |sum, i| sum + match i {
            DataTypes::IntegerType(IntegerType{value}) => { (*value - mean_value).pow(2) } 
            _ => 0
        });

        return DataTypes::IntegerType(IntegerType{value : return_value / denom });
    }
}

impl CalculateVariance for StringType {        
    fn variance(_ : &Vec<DataTypes>) -> DataTypes {
        return DataTypes::StringType(StringType{value : String::from("NAN")})
    }
}

// rolling statistics
//
// https://jonisalonen.com/2014/efficient-and-accurate-rolling-standard-deviation/
//
trait CalculateSimpleRollingMean {
    fn simple_rolling_mean(column : & Vec<DataTypes>, window : usize) -> Vec<DataTypes>;
}

impl CalculateSimpleRollingMean for FloatType {   
    fn simple_rolling_mean(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let fwin : f64 = window as f64;
        let mut rolling_avg : f64 = 0.0;
        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{ value : x.iter().zip(y).fold(rolling_avg, |acc, (a, b)| {
                    rolling_avg = (acc + (a.fvalue() - b.fvalue())) / fwin;
                    rolling_avg
                })
            })
        ).collect()
    }
}

impl CalculateSimpleRollingMean for IntegerType {
    fn simple_rolling_mean(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let iwin : i64 = window as i64;
        let mut rolling_avg : i64 = 0;
        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::IntegerType(IntegerType{ value : x.iter().zip(y).fold(rolling_avg, |acc, (a, b)| {
                    rolling_avg = (acc + (a.ivalue() - b.ivalue())) / iwin;
                    rolling_avg
                })
            })
        ).collect()
    }
}

impl CalculateSimpleRollingMean for StringType {        
    fn simple_rolling_mean(columns: &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        vec![DataTypes::StringType(StringType{value : String::from("NAN")});1]
    }
}

trait CalculateRollingStdDev {
    fn rolling_stddev(column : & Vec<DataTypes>, window : usize) -> Vec<DataTypes>;
}

impl CalculateRollingStdDev for FloatType {   
    fn rolling_stddev(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let fwin : f64 = window as f64;
        let fwinmo : f64 = fwin - 1.0;

        let mut rolling_avg : f64 = 0.0;
        let mut rolling_var : f64 = 0.0;

        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::FloatType(
                FloatType{ value : x.iter().zip(y).fold((0.0, rolling_avg, rolling_var, 0.0), |(nav, oav, var, stdev), (a, b)| {
                    rolling_avg = oav + ((a.fvalue() - b.fvalue()) / fwin);
                    rolling_var = ((b.fvalue() - a.fvalue()) * (b.fvalue() - rolling_avg + a.fvalue() - oav ) / fwinmo) + var;
                    (rolling_avg, oav, rolling_var, rolling_var.sqrt())
                }).3
            })
        ).collect()
    }
}

impl CalculateRollingStdDev for IntegerType {
    fn rolling_stddev(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let iwin : i64 = window as i64;
        let iwinmo : i64 = iwin - 1;

        let mut rolling_avg : i64 = 0;
        let mut rolling_var : i64 = 0;

        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::IntegerType(
                IntegerType{ value : x.iter().zip(y).fold((0, rolling_avg, rolling_var, 0), |(nav, oav, var, stdev), (a, b)| {
                    rolling_avg = oav + ((a.ivalue() - b.ivalue()) / iwin);
                    rolling_var = ((b.ivalue() - a.ivalue()) * (b.ivalue() - rolling_avg + a.ivalue() - oav ) / iwinmo) + var;
                    (rolling_avg, oav, rolling_var, (rolling_var as f64).sqrt() as i64)
                }).3
            })
        ).collect()
    }
}

impl CalculateRollingStdDev for StringType {        
    fn rolling_stddev(columns: &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        vec![DataTypes::StringType(StringType{value : String::from("NAN")});1]
    }
}

trait CalculateRollingVariance {
    fn rolling_variance(column : & Vec<DataTypes>, window : usize) -> Vec<DataTypes>;
}

impl CalculateRollingVariance for FloatType {   
    fn rolling_variance(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let fwin : f64 = window as f64;
        let fwinmo : f64 = fwin - 1.0;

        let mut rolling_avg : f64 = 0.0;
        let mut rolling_var : f64 = 0.0;

        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::FloatType(
                FloatType{ value : x.iter().zip(y).fold((0.0, rolling_avg, rolling_var), |(nav, oav, var), (a, b)| {
                    rolling_avg = oav + ((a.fvalue() - b.fvalue()) / fwin);
                    rolling_var = ((b.fvalue() - a.fvalue()) * (b.fvalue() - rolling_avg + a.fvalue() - oav ) / fwinmo) + var;
                    (rolling_avg, oav, rolling_var)
                }).2
            })
        ).collect()
    }
}

impl CalculateRollingVariance for IntegerType {
    fn rolling_variance(columns : &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        let iwin : i64 = window as i64;
        let iwinmo : i64 = iwin - 1;

        let mut rolling_avg : i64 = 0;
        let mut rolling_var : i64 = 0;

        columns.windows(window).zip(columns.windows(window).skip(1)).map(
            |(x, y)| DataTypes::IntegerType(
                IntegerType{ value : x.iter().zip(y).fold((0, rolling_avg, rolling_var), |(nav, oav, var), (a, b)| {
                    rolling_avg = oav + ((a.ivalue() - b.ivalue()) / iwin);
                    rolling_var = ((b.ivalue() - a.ivalue()) * (b.ivalue() - rolling_avg + a.ivalue() - oav ) / iwinmo) + var;
                    (rolling_avg, oav, rolling_var)
                }).2
            })
        ).collect()
    }
}

impl CalculateRollingVariance for StringType {        
    fn rolling_variance(columns: &Vec<DataTypes>, window : usize) -> Vec<DataTypes> {
        vec![DataTypes::StringType(StringType{value : String::from("NAN")});1]
    }
}

trait CalculateDiff {
    fn diff(column : & Vec<DataTypes>) -> Vec<DataTypes>;
}

impl CalculateDiff for FloatType {   
    fn diff(columns : &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{ value : (y.fvalue()-x.fvalue()) })
        ).collect()
    }
}

impl CalculateDiff for IntegerType {
    fn diff(columns : &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{ value : (y.fvalue()-x.fvalue()) })
        ).collect()
    }
}

impl CalculateDiff for StringType {        
    fn diff(columns: &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{value : (y.svalue().len() as f64 - x.svalue().len() as f64) })
        ).collect()
    }
}

trait CalculatePctChange {
    fn pct_change(column : & Vec<DataTypes>) -> Vec<DataTypes>;
}

impl CalculatePctChange for FloatType {   
    fn pct_change(columns : &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{value : (y.fvalue()-x.fvalue())/x.fvalue()})
        ).collect()
    }
}

impl CalculatePctChange for IntegerType {
    fn pct_change(columns : &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{value : (y.fvalue()-x.fvalue())/x.fvalue()})
        ).collect()
    }
}

impl CalculatePctChange for StringType {        
    fn pct_change(columns: &Vec<DataTypes>) -> Vec<DataTypes> {
        columns.iter().zip(columns.iter().skip(1)).map(
            |(x, y)| DataTypes::FloatType(FloatType{value : (y.svalue().len() as f64 - x.svalue().len() as f64)/x.svalue().len() as f64 })
        ).collect()
    }
}

trait CalculateCompare {
    fn cmp(r : &DataTypes, l : &DataTypes) -> bool;
}

impl CalculateCompare for FloatType {   
    fn cmp(r : &DataTypes, l : &DataTypes) -> bool {
        return r.fvalue() == l.fvalue()
    }
}

impl CalculateCompare for IntegerType {
    fn cmp(r : &DataTypes, l : &DataTypes) -> bool {
        return r.ivalue() == l.ivalue()
    }
}

impl CalculateCompare for StringType {        
    fn cmp(r : &DataTypes, l : &DataTypes) -> bool {
        return r.svalue() == l.svalue()
    }
}

type Series = Vec<DataTypes>;

pub struct DataFrame {
    labels : Vec<String>,
    columns : Vec<Series>,
}

impl DataFrame {

    pub fn new() -> DataFrame {
        DataFrame{labels : Vec::new() , columns : Vec::new() }
    }

    pub fn load_csv(&mut self, path : &Path) {
        let path_str : String = (*path.to_str().unwrap()).to_string();
        let contents : String = read_to_string(path_str).unwrap();
        let mut split_contents : std::str::Lines = contents.lines();
        let header = split_contents.next().unwrap();
        
        let fields : Vec<&str> = header.split(',').collect();
        for field in &fields {
            self.labels.push(String::from(*field));
        }

        let fields_len = &fields.len();
        self.columns = (0..*fields_len).map(|_x| Vec::new() ).collect();

        for ln in split_contents {
            let values : Vec<&str> = ln.split(',').collect();
            for (x, y) in (0..values.len()).zip(values) {
                let fvalue = y.parse::<f64>();
                let ivalue = y.parse::<i64>();
                if fvalue.is_ok() {
                    self.columns[x].push(DataTypes::FloatType(FloatType{value : fvalue.unwrap()}));
                }
                else if ivalue.is_ok() {
                    self.columns[x].push(DataTypes::IntegerType(IntegerType{value : ivalue.unwrap()}));
                }
                else {
                    self.columns[x].push(DataTypes::StringType(StringType{value : String::from(y)}));
                }
           }
        }
    }

    pub fn load_data(&mut self, data : & Vec<(&str, Series)>) {
        self.labels.resize(data.len(), "".to_string());
        self.columns.resize(data.len(), Series::new());

        for x in 0..data.len() {
            self.labels[x] = data[x].0.to_string();
            self.columns[x] = data[x].1.clone();
        }
    }

    pub fn add_column(&mut self, data : &(&str, Series)) {
        self.labels.push(data.0.to_string());
        self.columns.push(data.1.clone());
    }

    pub fn get_column_index(&self, column_name : &str) -> usize {
        self.labels.iter().position(|l| *l == column_name ).unwrap()
    }

    pub fn column_is_integer(&self, idx : usize) -> bool {
        match self.columns[idx].first().unwrap() {
            DataTypes::IntegerType(IntegerType{value : _}) => { true },
            _ => false
        }
    }

    pub fn column_is_float(&self, idx : usize) -> bool {
        match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { true },
            _ => false
        }
    }

    pub fn column_is_string(&self, idx : usize) -> bool {
        match self.columns[idx].first().unwrap() {
            DataTypes::StringType(StringType{value : _}) => { true },
            _ => false
        }
    }

    pub fn column(&self, column_name : &str) -> DataFrame {
        let idx = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::column },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::column },
            DataTypes::StringType(StringType{value : _}) => { StringType::column },
        };

        op(&self.columns[idx], &column_name.to_string())
    }

    pub fn series(&self, column_name : &str) -> Series {
        let idx = self.get_column_index(column_name);
        self.columns[idx].clone()
    }

    pub fn mean(&self, column_name : &str) -> DataTypes {
        let idx = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::mean },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::mean },
            DataTypes::StringType(StringType{value : _}) => { StringType::mean },
        };

        op(&self.columns[idx])
    }

    pub fn stddev(&self, column_name : &str) -> DataTypes {
        let idx = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::stddev },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::stddev },
            DataTypes::StringType(StringType{value : _}) => { StringType::stddev },
        };

        op(&self.columns[idx])
    }

    pub fn pstddev(&self, column_name : &str) -> DataTypes {
        let idx = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::pstddev },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::pstddev },
            DataTypes::StringType(StringType{value : _}) => { StringType::pstddev },
        };

        op(&self.columns[idx])
    }

    pub fn variance(&self, column_name : &str) -> DataTypes {
        let idx = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::variance },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::variance },
            DataTypes::StringType(StringType{value : _}) => { StringType::variance },
        };

        op(&self.columns[idx])
    }

    pub fn simple_rolling_mean(&self, column_name : &str, window : usize) -> DataFrame {
        let idx : usize = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::simple_rolling_mean },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::simple_rolling_mean },
            DataTypes::StringType(StringType{value : _}) => { StringType::simple_rolling_mean },
        };

        DataFrame{ labels : std::vec![column_name.to_string(),], columns : vec![op(&self.columns[idx], window),] }
    }

    pub fn rolling_stddev(&self, column_name : &str, window : usize) -> DataFrame {
        let idx : usize = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::rolling_stddev },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::rolling_stddev },
            DataTypes::StringType(StringType{value : _}) => { StringType::rolling_stddev },
        };

        DataFrame{ labels : std::vec![column_name.to_string(),], columns : vec![op(&self.columns[idx], window),] }
    }
    
    pub fn rolling_variance(&self, column_name : &str, window : usize) -> DataFrame {
        let idx : usize = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::rolling_variance },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::rolling_variance },
            DataTypes::StringType(StringType{value : _}) => { StringType::rolling_variance },
        };

        DataFrame{ labels : std::vec![column_name.to_string(),], columns : vec![op(&self.columns[idx], window),] }
    }

    pub fn diff(&self, column_name : &str) -> DataFrame {
        let idx : usize = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::diff },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::diff },
            DataTypes::StringType(StringType{value : _}) => { StringType::diff },
        };

        DataFrame{ labels : std::vec![column_name.to_string(),], columns : vec![op(&self.columns[idx],),] }
    }

    pub fn pct_change(&self, column_name : &str) -> DataFrame {
        let idx : usize = self.get_column_index(column_name);

        let op = match self.columns[idx].first().unwrap() {
            DataTypes::FloatType(FloatType{value : _}) => { FloatType::pct_change },
            DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::pct_change },
            DataTypes::StringType(StringType{value : _}) => { StringType::pct_change },
        };

        DataFrame{ labels : std::vec![column_name.to_string(),], columns : vec![op(&self.columns[idx],),] }
    }

    pub fn group_by(&self, column_names : Vec<&str>) -> Group {
        Group::new(self, column_names)
    }

    pub fn println(&self) {
        let col_count = self.labels.len();
        for i in 0..col_count {
            println!("{}", self.labels[i]);
            let col_len = self.columns[i].len();
            for j in 0..col_len {
                self.columns[i][j].println();
            }
        }
    }
}

pub struct BloomFilter {
    size : usize,    
    item_count : usize,
    hash_count : usize,
    false_probability : f64,
    bit_array : Vec<bool>,
}

impl BloomFilter {

    pub fn new(itemcount : usize, fprob : f64) -> BloomFilter {
        let log2 = 2_f64.log(10.0);
        let sz = (-(itemcount as f64 * fprob.log(10.0))/log2.powf(2.0)) as usize;
        let hc = ((sz / itemcount) as f64 * log2) as usize;
        BloomFilter{size : sz, item_count : itemcount, hash_count : hc, false_probability : fprob, bit_array : Vec::new() }
    }

    pub fn add<T>(&mut self, item : T)
        where T : Hash
    {
        if self.bit_array.len() < 1 {
            self.bit_array.resize(self.size, false);
        }

        let mut s = DefaultHasher::new();
        for i in 0..self.hash_count {
            item.hash(&mut s);
            let h = s.finish() as usize ^ i;
            let digest = (h as usize) % self.size;
            self.bit_array[digest] = true;
        }
    }

    pub fn contains<T>(& self, item : T) -> bool 
        where T : Hash
    {
        if self.bit_array.len() < 1 {
            return false;
        }

        let mut s = DefaultHasher::new();
        for i in 0..self.hash_count {
            item.hash(&mut s);
            let h = s.finish() as usize ^ i;
            let digest = (h as usize) % self.size;
            if self.bit_array[digest] == false {
                return false;
            }
        }

        return true
    }
}

pub struct Group<'a> {
    df : &'a DataFrame,    
    column_indices : Vec<usize>,
    indices : Vec< HashMap<DataTypes, Vec<usize>> >,
}

impl<'a> Group<'a> {

    pub fn new(df : &'a DataFrame, column_names : Vec<&str>) -> Group<'a> {
        let column_idxs : Vec<usize> = column_names.iter().map(|&x| df.get_column_index(x)).collect();

        let mut grp_indices : Vec< HashMap<DataTypes, Vec<usize>> > = vec![ HashMap::new(); column_idxs.len()];
        for (grp_idx, columns) in column_idxs.iter().enumerate().map(|(y,x)| (y, df.columns.get(*x)) ) {
            for (row_idx, coval) in columns.unwrap().iter().enumerate() {
                grp_indices.get_mut(grp_idx).unwrap().entry(coval.clone()).or_insert(Vec::new()).push(row_idx);
            }
        };

        Group{df : df, column_indices : column_idxs, indices : grp_indices, }
    }

    pub fn fields(&self) -> Vec< String >{
        self.column_indices.iter().map(|&x| self.df.labels[x].to_string()).collect()
    }
    
    pub fn size(&self) -> Vec< (String, Vec<(String, usize)>) > {
        self.column_indices.iter().zip(self.indices.iter()).map(
            |(x, y)| (
                String::from(self.df.labels.get(*x).unwrap()),
                y.iter().map( |z| (z.0.svalue(), z.1.len()) ).collect()
            )
        ).collect()
    }

    pub fn mean(&self) -> Vec< Vec<DataTypes> > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::mean },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::mean },
                DataTypes::StringType(StringType{value : _}) => { StringType::mean },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()

        }).collect()
    }

    pub fn stddev(&self) -> Vec< Vec<DataTypes> > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::stddev },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::stddev },
                DataTypes::StringType(StringType{value : _}) => { StringType::stddev },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()

        }).collect()
    }

    pub fn pstddev(&self) -> Vec< Vec<DataTypes> > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::pstddev },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::pstddev },
                DataTypes::StringType(StringType{value : _}) => { StringType::pstddev },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()

        }).collect()
    }

    pub fn variance(&self) -> Vec< Vec<DataTypes> > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::variance },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::variance },
                DataTypes::StringType(StringType{value : _}) => { StringType::variance },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()

        }).collect()
    }
    
    pub fn simple_rolling_mean(&self, window : usize) -> Vec< Vec< Vec<DataTypes> > > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::simple_rolling_mean },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::simple_rolling_mean },
                DataTypes::StringType(StringType{value : _}) => { StringType::simple_rolling_mean },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x, window)).collect()        
        }).collect()
    }

    pub fn rolling_stddev(&self, window : usize) -> Vec< Vec< Vec<DataTypes> > > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::rolling_stddev },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::rolling_stddev },
                DataTypes::StringType(StringType{value : _}) => { StringType::rolling_stddev },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x, window)).collect()        
        }).collect()
    }
    
    pub fn rolling_variance(&self, window : usize) -> Vec< Vec< Vec<DataTypes> > > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::rolling_variance },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::rolling_variance },
                DataTypes::StringType(StringType{value : _}) => { StringType::rolling_variance },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x, window)).collect()        
        }).collect()
    }

    pub fn diff(&self) -> Vec< Vec< Vec<DataTypes> > > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::diff },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::diff },
                DataTypes::StringType(StringType{value : _}) => { StringType::diff },
            };

            let data : Vec< Vec<DataTypes> > = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()
        }).collect()
    }
    
    pub fn pct_change(&self) -> Vec< Vec< Vec<DataTypes> > > {
        self.column_indices.iter().enumerate().map( |(i, &ci)| {

            let op = match self.df.columns[ci].first().unwrap() {
                DataTypes::FloatType(FloatType{value : _}) => { FloatType::pct_change },
                DataTypes::IntegerType(IntegerType{value : _}) => { IntegerType::pct_change },
                DataTypes::StringType(StringType{value : _}) => { StringType::pct_change },
            };

            let data : Vec<Vec<DataTypes>> = self.indices[i].iter().map( |(_, y)|
                y.iter().map(|&b| self.df.columns[ci][b].clone()).collect()
            ).collect();

            data.iter().map(|x| op(x)).collect()

        }).collect()
    }

    pub fn head(&self, num_rows : usize) {
        let width = 15;
        for lbl in self.labels.iter() { print!("{:^width$} ", lbl, width=width); }
        print!("\n");

        for row in 0..num_rows {
            print!("{}", row);
            for column in 0..self.columns.len() {
                match self.columns[column][row] {
                    DataTypes::FloatType(FloatType{value : _}) => {
                        print!(" {:^width$}", self.columns[column][row].fvalue(), width=width);
                    },
                    DataTypes::IntegerType(IntegerType{value : _}) => {
                        print!(" {:^width$}", self.columns[column][row].ivalue(), width=width);
                    },
                    DataTypes::StringType(StringType{value : _}) => { 
                        print!(" {:^width$}", self.columns[column][row].svalue(), width=width);
                    },
                };
            }
            print!("\n");
        }
    }
    
    pub fn print(&self) {
        for (i, k) in self.column_indices.iter().enumerate().map(|(y, x)| (y, self.df.labels.get(*x).unwrap())) {
            println!("{}", k);
            for (val, rows) in self.indices.get(i).unwrap().iter() {
                let k_str = val.svalue();
                print!("{}\t", k_str);
                for r in rows {
                    print!("{} ", r);
                }
            }
            println!("");
        }
    }
}
