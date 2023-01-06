#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisError, RedisResult, RedisString};

fn pi_archimedes(_: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }
   
    let times_arg = args[1].parse_integer();

    if times_arg.is_err() {
        return Err(RedisError::WrongType);
    }

    let times = times_arg.unwrap();

    if times <= 0 {
        return Err(RedisError::String("Wrong iteration count".to_string()));
    }

    if times > 16 {
        return Err(RedisError::String("More than 16 iterations not supported".to_string()));
    }

    let mut poly_len_squared:f64 = 2.0;
    let mut poly_sides = 4;
    
    for _i in 1..times {
        poly_len_squared = 2.0 - 2.0*(1.0 - poly_len_squared/4.0).sqrt();
        poly_sides *= 2;
    }

    let result = poly_sides as f64 * poly_len_squared.sqrt() / 2.0;
    Ok(redis_module::RedisValue::Float(result))

}

fn pi_gregory(_: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }
   
    let times_arg = args[1].parse_integer();

    if times_arg.is_err() {
        return Err(RedisError::WrongType);
    }

    let times = times_arg.unwrap();

    if times <= 0 {
        return Err(RedisError::String("Wrong iteration count".to_string()));
    }

    let mut result = 0.0;
    let mut sign = 1.0;

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
        ["pi.gregory", pi_gregory, "", 0, 0, 0],
        ["pi.archimedes", pi_archimedes, "", 0, 0, 0],
    ],
}
