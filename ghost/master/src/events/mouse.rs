use crate::events::common::*;
use crate::variables::GlobalVariables;
use std::collections::HashMap;

pub type MouseDialogs = HashMap<String, Vec<String>>;

pub fn wheel_dialogs(_vars: &mut GlobalVariables) -> MouseDialogs {
    let mut dialogs: MouseDialogs = HashMap::new();

    // 0skirtup
    let mut zero_skirt_up = Vec::new();
    zero_skirt_up.extend(all_combo(&vec![
        vec!["h2244402……！\\nh1241102\\_w[500]".to_string()],
        vec![
            "h1111204……いいもの見たって顔してる。h1111209屈辱だわ。".to_string(),
            "h1111205……ああ、ひどい人。h1111209泣いてしまいそうだわ。".to_string(),
            "h1111204……悪餓鬼。".to_string(),
        ],
    ]));
    dialogs.insert("0skirtup".to_string(), zero_skirt_up);

    dialogs.insert("0shoulderdown".to_string(), vec!["抱き寄せる".to_string()]);

    dialogs
}
