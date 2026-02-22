use ahl_lang::{AhlEngine, Value};
use crate::game_api;
use crate::units::*;

static mut ENGINE_PTR: *mut AhlEngine = std::ptr::null_mut();

pub fn bind_engine(engine: &mut AhlEngine) {
    unsafe { ENGINE_PTR = engine as *mut AhlEngine; }
}

/// Register all native game functions and load the turn script.
pub fn setup_engine(engine: &mut AhlEngine, script_path: &str) {
    // -- get_units() → [[x, y, hp, kind], ...]
    engine.register_fn("get_units", |_args| {
        let units = game_api::get_units();
        let list: Vec<Value> = units.iter().map(|u| {
            let kind_str = match u.kind {
                UnitKind::Archer => "archer",
                UnitKind::Fighter => "fighter",
                UnitKind::Catapult => "catapult",
                UnitKind::King => "King",
            };
            Value::List(vec![
                Value::Int(u.position[0] as i64),
                Value::Int(u.position[1] as i64),
                Value::Int(u.hp as i64),
                Value::Str(kind_str.to_string()),
            ])
        }).collect();
        Ok(Value::List(list))
    });

    // -- unit_count() → int
    engine.register_fn("unit_count", |_args| {
        let units = game_api::get_units();
        Ok(Value::Int(units.len() as i64))
    });

    // -- move_unit(index, direction_string)
    engine.register_fn("move_unit", |args| {
        if args.len() != 2 {
            return Err("move_unit(index, direction) expects 2 arguments".to_string());
        }
        let idx = match &args[0] {
            Value::Int(n) => *n as usize,
            _ => return Err("move_unit: first argument must be int".to_string()),
        };
        let dir_str = match &args[1] {
            Value::Str(s) => s.clone(),
            _ => return Err("move_unit: second argument must be a direction string".to_string()),
        };
        let dir = parse_direction(&dir_str)?;
        let count = game_api::get_units().len();
        if idx >= count {
            return Err(format!("move_unit: index {} out of bounds ({} units)", idx, count));
        }
        game_api::move_unit(idx, dir);
        Ok(Value::Nil)
    });

    // -- spawn_unit(kind_string, x, y)
    engine.register_fn("spawn_unit", |args| {
        if args.len() != 3 {
            return Err("spawn_unit(kind, x, y) expects 3 arguments".to_string());
        }
        let kind_str = match &args[0] {
            Value::Str(s) => s.clone(),
            _ => return Err("spawn_unit: first argument must be a kind string".to_string()),
        };
        let x = match &args[1] {
            Value::Int(n) => *n as u8,
            _ => return Err("spawn_unit: x must be int".to_string()),
        };
        let y = match &args[2] {
            Value::Int(n) => *n as u8,
            _ => return Err("spawn_unit: y must be int".to_string()),
        };
        let pos = [x, y];
        let unit = match kind_str.as_str() {
            "archer" => Unit::archer(pos),
            "fighter" => Unit::fighter(pos),
            "catapult" => Unit::catapult(pos),
            _ => return Err(format!("spawn_unit: unknown unit kind '{}'", kind_str)),
        };
        game_api::spawn_unit(unit, x, y);
        Ok(Value::Nil)
    });

    // -- get_terrain(x, y) → string
    engine.register_fn("get_terrain", |args| {
        if args.len() != 2 {
            return Err("get_terrain(x, y) expects 2 arguments".to_string());
        }
        let x = match &args[0] {
            Value::Int(n) => *n as usize,
            _ => return Err("get_terrain: x must be int".to_string()),
        };
        let y = match &args[1] {
            Value::Int(n) => *n as usize,
            _ => return Err("get_terrain: y must be int".to_string()),
        };
        let terrain = game_api::get_terrain(x, y)?;
        Ok(Value::Str(terrain))
    });

    // Load and execute the script (defines functions like on_turn)
    let source = std::fs::read_to_string(script_path)
        .unwrap_or_else(|e| panic!("Failed to load script '{}': {e}", script_path));
    engine.exec(&source)
        .unwrap_or_else(|e| panic!("Script error in '{}': {e}", script_path));
}

/// Re-load the script source from the editor and run on_turn().
pub fn reload_and_run(source: &str) -> Result<(), String> {
    unsafe {
        if ENGINE_PTR.is_null() { return Err("ENGINE_PTR is not initialized".to_string()); }
        let engine = &mut *ENGINE_PTR;
        engine.exec(source)?;
        engine.call_fn("on_turn", vec![])?;
    }
    Ok(())
}

fn parse_direction(s: &str) -> Result<Direction, String> {
    match s {
        "up" => Ok(Direction::Up),
        "down" => Ok(Direction::Down),
        "left" => Ok(Direction::Left),
        "right" => Ok(Direction::Right),
        "up_right" => Ok(Direction::UpRight),
        "up_left" => Ok(Direction::UpLeft),
        "down_right" => Ok(Direction::DownRight),
        "down_left" => Ok(Direction::DownLeft),
        _ => Err(format!("unknown direction: '{}' (use up/down/left/right/up_left/up_right/down_left/down_right)", s)),
    }
}
