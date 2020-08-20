//  Copyright (c) 2020 Christopher Taylor
//
//  SPDX-License-Identifier: BSL-1.0
//  Distributed under the Boost Software License, Version 1.0. (See accompanying
//  file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
use std::fs::read_to_string;
use std::path::Path;
use std::vec;

use framedata::{DataFrame};

fn main() {
    let path_str : String = String::from("PRECIP_HLY_sample_csv.csv");
    let path = Path::new(&path_str);

    let mut df : DataFrame = DataFrame::new();
    
    df.load_csv(&path);
    df.println();

    let groupby = df.group_by(vec!["ELEVATION", "STATION"]);

    let groupbysize = groupby.size();

    for (k,vs) in groupbysize {
        println!("{}", k);
        for (dt, v) in vs {
            println!("{} {}", dt, v);
        }
        println!("");
    }

    groupby.print();
    println!();

    let groupbymean = groupby.mean();
    println!("mean");
    for x in groupbymean {
        for y in x {
            y.println();
        }
    }

    let groupbyrollingmean = groupby.simple_rolling_mean(2);
    println!("rolling mean");
    for x in groupbyrollingmean {
        for y in x {
            for z in y {
                z.println();
            }
        }
    }

    let groupbydiff = groupby.diff();
    println!("diff");
    for x in groupbydiff {
        for y in x {
            for z in y {
                z.println();
            }
        }
    }

    let groupbypctchange = groupby.pct_change();
    println!("pct_change");
    for x in groupbypctchange {
        for y in x {
            for z in y {
                z.println();
            }
        }
    }

    for f in groupby.fields(){
        println!("{}", f);
    }
}
