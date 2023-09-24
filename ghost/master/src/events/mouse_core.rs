use crate::events::common::*;
use crate::events::menu::on_menu_exec;
use crate::events::mouse::*;
use crate::variables::{Direction, GlobalVariables};

use shiorust::message::{Response, *};
use std::time::SystemTime;

// 撫での蓄積値の閾値: この値を超えたら撫でイベントを発生させる
const NADE_THRESHOLD: i32 = 12;

// 撫でをカウントする間隔(ms): 最後の撫でからこの時間が経つまで撫でをカウントしない
const NADE_DURATION: u128 = 100;

// 撫での蓄積値がリセットされるまでの時間(ms)
const NADE_LIFETIME: u128 = 3000;

// ホイールの蓄積値の閾値: この値を超えたらホイールイベントを発生させる
const WHEEL_THRESHOLD: i32 = 3;

// ホイールの蓄積値がリセットされるまでの時間(ms)
const WHEEL_LIFETIME: u128 = 3000;

pub fn on_mouse_wheel(req: &Request, vars: &mut GlobalVariables) -> Response {
    let refs = get_references(req);
    if refs[4] == "" {
        new_response_nocontent()
    } else {
        let now = SystemTime::now();
        let dur = now
            .duration_since(vars.volatility.last_wheel_count_unixtime)
            .unwrap()
            .as_millis();

        let d: Direction;
        if refs[2].parse::<i32>().unwrap() > 0 {
            // up
            d = Direction::Up;
        } else {
            // down
            d = Direction::Down;
        }

        if vars.volatility.last_wheel_part != refs[4]
            || vars.volatility.wheel_direction != d
            || dur > WHEEL_LIFETIME
        {
            vars.volatility.wheel_counter = 1;
        } else {
            vars.volatility.wheel_counter += 1;
        }

        if vars.volatility.wheel_counter >= WHEEL_THRESHOLD {
            vars.volatility.wheel_counter = 0;
            let info = format!("{}{}{}", refs[3], refs[4], d.to_str());
            new_response_from_mouse_dialogs(&wheel_dialogs(vars), info, vars, true)
        } else {
            vars.volatility.last_wheel_count_unixtime = now;
            vars.volatility.last_wheel_part = refs[4].to_string();
            vars.volatility.wheel_direction = d;
            new_response_nocontent()
        }
    }
}

pub fn on_mouse_double_click(req: &Request, vars: &mut GlobalVariables) -> Response {
    let refs = get_references(req);
    if refs[4] == "" {
        on_menu_exec(req, vars)
    } else {
        new_response_with_value(refs[4].to_string(), vars, true)
    }
}

pub fn on_mouse_click_ex(req: &Request, vars: &mut GlobalVariables) -> Response {
    let refs = get_references(req);
    if refs[5] == "middle" {
        new_response_with_value(format!("{}中クリック", refs[4]), vars, false)
    } else {
        new_response_nocontent()
    }
}

pub fn on_mouse_move(req: &Request, vars: &mut GlobalVariables) -> Response {
    let refs = get_references(req);
    if refs[4] == "" {
        new_response_nocontent()
    } else {
        let now = SystemTime::now();
        if vars.volatility.last_nade_part == refs[4] {
            let dur = now
                .duration_since(vars.volatility.last_nade_count_unixtime)
                .unwrap()
                .as_millis();
            if dur > NADE_LIFETIME {
                vars.volatility.nade_counter = 1;
                vars.volatility.last_nade_count_unixtime = now;
            } else if dur >= NADE_DURATION {
                vars.volatility.nade_counter += 1;
                vars.volatility.last_nade_count_unixtime = now;
            }
            debug!("{} {} {}", refs[4], dur, vars.volatility.nade_counter);
        } else {
            vars.volatility.nade_counter = 1;
        }
        vars.volatility.last_nade_part = refs[4].to_string();
        if vars.volatility.nade_counter > NADE_THRESHOLD {
            vars.volatility.nade_counter = 0;
            new_response_with_value(refs[4].to_string() + "なで", vars, true)
        } else {
            new_response_nocontent()
        }
    }
}
