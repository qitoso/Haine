use crate::events::common::*;
use shiorust::message::{Response, *};

pub fn on_boot(_req: &Request) -> Response {
  // \1のサーフェスを\0に重ねて固定する
  let init_tag = "\
    \\1\\![reset,sticky-window]\
    \\![set,alignmenttodesktop,free]\
    \\![move,--X=0,--Y=0,--time=0,--base=0]\
    \\![set,sticky-window,1,0]";

  let talks = all_combo(&vec![
    vec!["h1113105\\1今日も、霧が濃い。".to_string()],
    vec![
      "h1113105……h1113101\\_w[300]h1113201あら、h1111204いらっしゃい、{user_name}。".to_string(),
    ],
  ]);
  let v = format!("{}{}", init_tag, choose_one(&talks, false).unwrap(),);
  new_response_with_value(v, true)
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
        \
        h1000000────前略、現在私は幽霊屋敷に来ていた。\\n\
        「カンテルベリオの幽霊屋敷」\\n\
        その手のマニアの間では有名な廃墟だ。\\n\
        古い洋館で、周囲は小さな庭園に囲まれている。\\n\
        何度か取り壊しの話が出たが、原因不明の事故が続いたため、現在も残っているいわくつきの場所。\\n\
        \\n\
        ここには幽霊が出るという。\\n\
        目撃情報は多々あり、そして、見た者の多くが病気や事故に遭っている。\\n\
        それは呪いによるものなのか、それともただの偶然か。\\n\
        何にせよ、じつに興味を惹かれた。\\n\
        そしてついに今日、私はそこを訪れている。\\x\
        \
        \\1霧の濃い日だ。カンテルベリオではままあることらしい。\\n\
        数メートル先も見通せないほどの霧だが、不思議と濡れた感じがしない。\\n\
        むしろ乾ききっているような気がして、どことなく不安な気分になる。\\n\
        ……廃墟を目の前にして萎縮しているのだろうか。\\n\
        いや、ここまで来たのだから、引き返すわけにはいかない。\\n\
        私は歩き出した。\\x\
        \
        \\1玄関から入って、廊下を進む。\\n\
        埃っぽさは無いが、しかし生活感もない。\\n\
        窓をきっちりと覆っている重たげなカーテンは、長いあいだ開いていないようだった。\\n\
        \\n\
        しばらく歩き、客間へと入ったときだった。\\n\
        人の気配。\\_w[1000]\\n\
        部屋の奥の方を見やると、それは静かに、近づいてきていた。\\n\
        『(いた……！)』\\n\
        \
        \\0\\![bind,シルエット,黒塗り1,1]h1111201\\_w[1000]\
        ────また一人、新しいお客様。\\n\
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
        h1111201好奇とは尊く、そして卑しいものだわ。\\n\
        あなたもそう思わない？\\n\\n\
        \
        \\1かすかな衣擦れの音とともに暗がりから現れた人影は、少女のようだった。\\n\
        いや……人影というにはひどく違和感がある。\\n\
        暗がりであやしく光る真っ赤な瞳、死人のように血の気のない肌。\\n\
        人の形をしてはいるが、明らかに……。\\n\\n\
        \
        \\0\\![bind,シルエット,黒塗り4,0]\
        h1111204人間ではない。\
        \
        \\1！\
        \
        \\x\
        \
        h1111209そうね、人間はこんなに赤い目や白い肌をしていない。\\n\
        わかっていても、幽霊にとって人の姿を維持するのはとても難しいことなの。\\n\\n\
        \
        \\1幽霊、まさか本当に……。\\n\\n\
        \
        h1111204目の前にいるのだから、あなたは信じるしかないのよ。\\n\
        h1111309「カンテルベリオの幽霊」、おおかたあの噂を聞いてきたのでしょう。\\n\\n\
        h1112304おめでとう、私が件の幽霊よ。\\n\
        \\x\
        \
        ……h1111209さて、その嬉しそうな顔を見るに、あなたは私たちに興味があるようね。\\n\
        オカルトフリークが廃墟探索、珍しいことじゃないわ。\\n\
        h1111304幽霊、魅力的だものね？\\n\
        h1111206その自由さ、存在の不思議さ、\\n\
        あるいは、h1111306死そのものへのあこがれ。\\n\
        あなたは少なからず、それらを抱いていた。\\n\
        h1111209だからあなたはここに来たのでしょう。\\n\\n\
        \
        h1111209……h1111109……h1111106……。\\n\
        h1121306ふん、あなたのような輩はいつもそう。\
        \\x\
        \
        h1111101…………h1111301ねえ、考えればわかるはずよ。\\n\
        自分のテリトリーを悪戯半分で踏み荒らされる私の気持ち。\\n\
        \\![bind,シルエット,黒塗り2,1]\\![bind,ex,こわいかお,1]\
        " + &shake("あなた、考えたことはある？") + "\
        \
        \\1ぞくり、と背筋に冷たいものが走る。\\n\
        少女の雰囲気が変わった。\\n\
        理解するより先に、身体ががたがた震え出す。\
        \\x\
        \
        \\1怖い。怖い。頭の中からそれ意外の思考が消える。\\n\
        ……耐え難い恐怖ののち、今さら、後悔の念が湧いてくる。\\n\
        少女がこちらへ怒りを向けているのは明白だ。\\n\\n\
        当然だ。居場所を荒らされるのを嫌うのは、人間も幽霊も同じ。\\n\
        それを知りながら、私は自分の興味のために、\\n\
        この少女の居場所を踏み荒らしたのだ。\\n\\n\
        \
        h1111201……怖い？怖いのでしょうね。\\n\
        怨念とはそういうものよ。\\n\
        h1111206ふざけた輩には病を憑けて帰しているのだけど、\\n\
        h1111204あなたは……h1111309ふふ、どうしてあげようかしら。\\n\\n\
        \
        \\1『……ご、ごめんなさい……。』\\_w[1000]\\n\
        呟くように口をついで出たのは、子供じみた単純な謝罪だった。\\n\
        言ったあとで、これでは彼女の神経を逆撫でするだけだと後悔がよぎる。\\n\
        ああ、ああ、私はどうして……。\
        \\x
        \
        \\0h1111109\\![bind,ex,こわいかお,0]\\c…………。h1123207\\![bind,シルエット,,0]くく、ふふふ。\\n\
        h1123201ああ、おかしい。h1123204逃げずに許しを請うなんて。\\n\\n\
        h1123209ねえ、あなた。\\n\
        あなたのその狭い視野も、\\n\
        臆病なくせに愚鈍な態度も、\\n\
        h1123305まるで親に置いていかれた子犬みたいだわ。\\n\\n\
        \
        \\1……空気が、緩んだ……？\
        \\x\
        h1111201いいわ、気に入った。h1111204許してあげる。\\n\\n\
        \
        \\1た、助かった……。\\n\\n\
        \
        h1111209ただし。h1111201その代わりにあなた、しばらくここに居なさい。\\n\\n\
        \
        \\1φ！？\\n\\n\
        \
        h1111201あら、代償もなしに帰れると思ったの？\
        h1121204残念でした。h1111209まあ、あなたに払えないものを要求するつもりはないわ。\\n\\n\
        \
        h1111206……私は、ここでずっと、待っているのよ。\\n\
        待つことは嫌いではないけれど、話し相手がいなくて退屈していたところだったの。\\n\
        h1111207だから、すこしお話をしましょう。\\n\\n\
        h1111201私はハイネ。カンテルベリオの幽霊。\\n\
        h1111209この家の主として、あなたの訪問を歓迎します。\\n\
        h1111204せいぜい楽しませてちょうだいね？\\n\\nh1000000\
        \
        \\1客間の椅子を指さすハイネの手。\\n\
        席につけということだ。……従うしかない。\
        \
        \\0……h1111203さて。h1111201ところであなた、\\n\
        h1111204幽霊の淹れるお茶を飲んだことはあるかしら？h1111207\
    ";

  new_response_with_value(m, true)
}

pub fn on_vanish_selected(_req: &Request) -> Response {
  new_response_nocontent()
}

fn shake(text: &str) -> String {
  let mut s = String::new();
  let shakes = vec![(10, 10), (-14, -14), (4, 4)];
  for (i, c) in text.chars().enumerate() {
    if i < shakes.len() {
      s.push_str(&format!(
        "\\![moveasync,--X={},--Y={},--base=me]{}",
        shakes[i].0, shakes[i].1, c
      ));
    } else {
      s.push(c);
    }
  }
  s
}
