#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisError, RedisResult, RedisString};

fn pi_approx(_: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let mut result = 0.0;
    let mut sign = 1.0;
    
    let times_arg = args[1].parse_integer();

    if times_arg.is_err() {
        return Err(RedisError::WrongType);
    }

    let times = times_arg.unwrap();

    for d in 0..times {
        let df = d as f64;
        result += sign * (1.0/(2.0*df + 1.0));
        sign = -1.0 * sign;
    }

    result = result * 4.0;

    Ok(redis_module::RedisValue::Float(result))
}

//////////////////////////////////////////////////////

redis_module! {
    name: "pi",
    version: 1,
    data_types: [],
    commands: [
        ["pi", pi_approx, "", 0, 0, 0],
    ],
}
