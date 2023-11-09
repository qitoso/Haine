use crate::events::common::*;
use shiorust::message::{Response, *};

pub fn on_boot(_req: &Request) -> Response {
  new_response_with_value("h1111201Hello".to_string(), true)
}

pub fn on_close(_req: &Request) -> Response {
  let talks = all_combo(&vec![
    vec!["h1111209".to_string(), "h1111207".to_string()],
    vec!["あなたに".to_string()],
    vec![
      "すばらしき朝".to_string(),
      "蜜のようなまどろみ".to_string(),
      "暗くて静かな安らぎ".to_string(),
      "良き終わり".to_string(),
      "孤独と救い".to_string(),
    ],
    vec!["がありますように。\\nh1111204またね、{user_name}。\\_w[1200]".to_string()],
  ]);
  new_response_with_value(choose_one(&talks, true).unwrap() + "\\-", true)
}

pub fn on_first_boot(_req: &Request) -> Response {
  let m = "".to_string() + "\
        \\0\\![bind,シルエット,黒塗り1,1]h1111201\\_w[1000]\
        ────また一人、新しいお客人。\\n\
        \
        \\![bind,シルエット,黒塗り2,1]\
        招いてもいないのにφ、\\_w[750]\
        \\![anim,offset,510002,0,-100]\\n\
        あなたたちはここを訪れるφ。\\_w[750]\\n\
        \\![anim,offset,510002,0,-150]\
        私があなたたちを歓迎する保証なんてφ、\\_w[750]\\n\
        \\![anim,offset,510002,0,-250]\
        どこにもないのに。\\n\
        \
        \\![bind,シルエット,黒塗り4,1]\
        ねえ、あなたは、どうしてここへ来ようと思ったの？\\n\
        \
        \\1かすかな衣擦れの音とともに暗がりから現れた人影は、少女のようだった。\\n\
        いや……人影というにはひどく違和感がある。\\n\
        暗がりであやしく光る真っ赤な瞳、死人のように血の気のない肌。\\n\
        人の形をしてはいるが、明らかに……。\\n\\n\
        \
        \\0\\![bind,シルエット,黒塗り4,0]\
        h1111204人間ではない。\\n\\n\
        \
        \\1！\\n\\n\
        \
        \\x\
        \
        h1111209そうね、人間はこんなに赤い目や白い肌をしていない。\\n\
        わかっていても、幽霊にとって人の姿を維持するのはとても難しいことなの。\\n\\n\
        \
        \\1幽霊、まさか本当に……。\\n\\n\
        \
        h1111204目の前にいるのだから、あなたは信じるしかないのよ。\\n\
        h1112304「カンテルベリオの幽霊」、おおかたあの噂を聞いてきたのでしょう。\\n\\n\
        h1111304おめでとう、私が件の幽霊よ。\\n\
        \\x\
        \
        ……h1111209さて、その嬉しそうな顔を見るに、あなたは幽霊に興味があるようね。\\n\
        オカルトフリークが廃墟探索、珍しいことじゃないわ。h1111304幽霊、魅力的だものね？\\n\
        h1111203その自由さ、存在の不思議さ、\\n\
        あるいは、h1111206死そのものへのあこがれ。\\n\
        あなたは少なからず、それらを抱いていた。\\n\
        h1111209だからあなたはここに来たのでしょう。\\n\\n\
        \
        h1111209……h1111109……h1111106……。\\n\
        h1121306ふん、あなたのような輩はいつもそう。\\n\\n\
        \\x\
        \
        h1111101…………h1111301ねえ、考えればわかるはずよ。\\n\
        自分のテリトリーを悪戯半分で踏み荒らされる私の気持ち。\\n\
        \\![bind,シルエット,黒塗り2,1]\\![bind,ex,こわいかお,1]h1111101\
        " + &shake("あなた、考えたことはある？") + "\\n\\n\
        \
        \\1ぞくり、と背筋に冷たいものが走る。\\n\
        少女の雰囲気が変わった。身体ががたがた震え出す。\\n\
        \\x\
        \
        \\1……耐え難い恐怖とともに、今さら、後悔の念が湧いてくる。\\n\
        少女がこちらへ怒りを向けているのは明白だ。\\n\\n\
        当然だ。居場所を荒らされるのを嫌うのは、人間も幽霊も同じだ。\\n\
        それを知りながら、私は、自分の興味のために、\
        この少女の居場所を荒らしてしまったのだ。\\n\\n\
        \
        h1111206ふざけた輩には病を憑けて帰しているのだけど、\\n\
        h1111204あなたは……h1111309ふふ、どうしてあげようかしら。\\n\\n\
        \
        \
        \\1『……ご、ごめんなさい……。』\\_w[1000]\\n\
        口をついで出たのは、子供じみた単純な謝罪だった。\\n\
        言ったあとで、これでは彼女の神経を逆撫でするのではないかと後悔がよぎる。\\n\
        ああ、ああ、私はどうして……。\
        \\x
        \
        \\0\\![bind,ex,こわいかお,0]h1111109\\c…………。\\![bind,シルエット,,0]h1123207くく、ふふふ。\\n\
        h1123201ああ、おかしい。h1123204逃げずに許しを請うなんて。\\n\\n\
        h1123209ねえ、あなた。\\n\
        あなたのその狭い視野も、臆病なくせに愚鈍な態度も、h1123305まるで親に置いていかれた子犬みたいだわ。\\n\\n\
        \
        \\1……空気が、緩んだ……？\\n\\n\
        \\x\
        h1111201いいわ、気に入った。h1111204許してあげる。\\n\
        h1111209ただし。h1111201その代わりにあなた、しばらくここに居なさい。\\n\\n\
        h1111206……私はここでずっと、待っているのよ。\\n\
        待つことは嫌いではないけれど、最近は話し相手がいなくて退屈していたところだったの。\\n\
        h1111207ねえ、すこしお話をしましょう。\\n\\n\
        h1111201私はハイネ。カンテルベリオの幽霊。\\n\
        h1111209この家の主として、あなたの訪問を歓迎します。\\n\
        h1111204……せいぜい楽しませてちょうだいね？\\n\
        \\n\\n\
        h1111201……ところであなた、h1111204幽霊の淹れるお茶を飲んだことはあるかしら？h1111207\\n\\n\
    ";

  new_response_with_value(m, true)
}

fn shake(text: &str) -> String {
  let mut s = String::new();
  let shakes = vec![(10, 10), (-14, -14), (4, 4)];
  for (i, c) in text.chars().enumerate() {
    if i < shakes.len() {
      s.push_str(&format!("\\![moveasync,--X={},--Y={},--base=me]{}", shakes[i].0, shakes[i].1, c));
    } else {
      s.push(c);
    }
  }
  s
}
