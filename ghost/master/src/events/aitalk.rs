use crate::events::common::*;
use crate::variables::get_global_vars;
use once_cell::sync::Lazy;
use shiorust::message::{Request, Response};

static TALKS: Lazy<Vec<String>> = Lazy::new(|| {
  vec![
    format!("\
        h1111109未練もなく、しかし現世に留まっている魂。\\n\
        h1111105あるべきでないものはやがて消滅する。\\n\
        h1111205私は、それを望んでいるの。
    "),

    format!("\
        h1111205危険と隣り合わせだからこそ、世界は美しい。
        身を損なう心配がなくなっては、美しさが心を打つこともない。
        h1121205ただただ平坦な、揺らがぬ水面があるだけ。\\nh1121209それはやがて淀み、腐る。
    "),

    format!("\
    h1111205\\1ハイネが読んでいた本を閉じ、静かに語り始める。\\n\
    h1111205ある詩人が言っていたのよ。「死は必ずしも最悪の運命ではない。h1111209もっと恐ろしい運命がある」と。\
    {}\
    h1111201\\n\\n永遠に続く孤独よ。h1111206まあ、それの示すところは身をもって知っているわ。彼の言葉を借りるまでもなくね。\
    ", user_talk("それは…？","", false)),

    "\
    h1111105人生に変化は付きもの……けれど、h1111109停滞はそれ以上。\\n\
    一度立ち止まってしまうと、空気は一瞬で淀んで、身動きがとれなくなってしまうのよ。\\n\
    それは倦怠とも違う、鈍い痛み。あなたも経験したこと、あるんじゃないかしら。\\n\
    h1111206そうなったときは、多少無理にでも変化を取り入れるほうがいい。\\n\
    ……h1111209たとえなにかを破壊することになるとしても、何も出来ないよりはずっとましよ。\
    ".to_string(),

    format!("\
    h1111205悲しい。ここに縛られていることが、ではない。私が見ることのできない世界のこと。\\n\
    ……h1111209何も、知ることができないの。老人の、幼子の、男の、女の、見る世界を、\\n\
    そのすべてを私がこの身で知ることはかなわない。h1111205決して、……h1111209決して。\\n\
    それが、悲しくて、悔しくて、気が狂いそうになる。\\n\
    h1122305だって、せっかくこの世に生まれたのに。こんなにも自由なのに。……この手で、何をすることもできるはずなのに！\\n\\n\\_w[1200]\
    {}\
    \\0……h1121209ごめんなさい。\\nh1111205あなたに言っても詮無いことだわ。忘れてちょうだい。\
    ",
    user_talk("ハイネ……。","思わず、声が漏れる。", false)),

    format!("\
    h1111202霧を操作すれば外見を変えることもできるの。h1111207……見ていて。\\n\\n\
    h1000000{}\\n\
    \\0ふふ、あなたが望む姿になるわよ。髪も、身長も、年齢だって変えられるんだから。\\n\\n\
    h1111209\\1瞬きしたとき、彼女はまた元の姿に戻っていた。\
    h1111204あまり大きく変化すると自己認識が揺らいでしまうから、限界もあるのだけど。\\n\
    h1111207こういう戯れもたまにはいいでしょう？\
    ",
    user_talk("……髪が伸びてる！","霧が彼女を包んだかと思うと、その姿は一瞬で変わった。", true)),

    "\
    h1111205見慣れたはずの場所にいながら、いつもと違う道に迷いこんだことはある？\\n\
    \\n\
    h1111204もしそうなったら、「\\_a[Yomotsuhegui,ヨモツヘグイって？]ヨモツヘグイ\\_a」、\
    つまり、食べ物には常に注意しなさい。\\n\
    h1111205一度だけなら、は許されない。\\n\
    それがすべてを変えてしまうような落とし穴がこの世にはあるの。\
    ".to_string(),

    "\
    h1111209生と死はグラデーションか、二極対立か。\\n\
    議論に興味はないけれど、h1111205どちらにせよ、決定的な瞬間があるはず。\\n\
    h1111205その、死の瞬間。正直に言うと、私は、それがどうしようもなく愛しいの。命が命でなくなり、身体が陳腐な肉の塊になる、その一瞬が愛しくてたまらない。\\n\\n\
    h1111206私はきっと正しくない。あなたの目には、贔屓目にも綺麗には写らない。それは、悲しいことだけれど。\\n\\n\
    h1111309だけど、それがなんだというの。倫理は私を救わない。あなたの心を私が握る必要もない。私が諦める理由にはならないわ。\\n\
    h1111304それを諦められるくらい、私は、愛を諦められないのだから。\\n\\n\
    h1111205諦めなければ、いつか。\\n\
    h1111205……あんな啖呵を切った手前でこれを言うのは、傲慢だけれど。\\n\
    h1111207どうか、見届けていて、{user_name}。\
    ".to_string(),

    "\
    h1111209霧がなければ生きられない。\\n\
    霧があるから生きている。\\n\
    私は霧に生かされている。\\n\
    h1111205私に明日は、\\_a[Nolonger]もう来ない。\\_a\
    ".to_string(),

    "\
    h1111209あなたたちが歩いている姿を、いつも窓から見ているの。\\n\
    h1111204いつも何かをして、どこかへ向かっている。\\n\
    h1111207羨ましいわ。h1111207私は\\_a[Fastened,どういうこと？]見ていることしかできない\\_aから、なおさら。\
    ".to_string(),

    "\
    h1111209霧の中では、天気はいつも同じように見えるわね。\\n\
    h1111209晴れているのか、曇っているのか、分かりはしないけれど。\\n\
    h1111206唯一、雨は分かるでしょう？だから好きなの。\\n\
    h1111201あなたはどの天気が好き？\
    ".to_string(),

    "\
    h1111209黒死病が蔓延していたとき、問題になっていたのがいわゆる「早すぎた埋葬」。\\n\
    h1111205ある技師は生き埋めにされる恐怖から逃れるため、棺の内側から埋葬者が生きていることを知らせる仕組みがついた棺を開発したの。\\n\
    h1111204彼、デモンストレーションのために自ら生き埋めになってみせたそうよ。\\n\
    \\n\
    h1211209支援者に強制でもされたのかしら。それとも、自分からそんな真似を？なんにせよ、h1211205狂気の沙汰ね。\
    ".to_string(),

    "\
    h1111205ある本を最初に読んだときの感動と、何度も読み返して全て見知ったゆえの倦み。\\n\
    どちらがその本の真の印象かしら？\\n\\n\
    h1111209私はどちらも正しいと思う。印象なんてその時々で変わるもので、\\n\
    h1111205一つに定まることなんて稀だもの。\\n\\n\
    そう。正反対の感想を同時に抱いたっていいのよ。\
    ".to_string(),
  ]
});

pub fn version(_req: &Request) -> Response {
  new_response_with_value(String::from(env!("CARGO_PKG_VERSION")), false)
}

pub fn on_ai_talk(_req: &Request) -> Response {
  let vars = get_global_vars();
  new_response_with_value(
    choose_one(
      &TALKS,
      vars.volatility.idle_seconds > vars.volatility.idle_threshold,
    )
    .unwrap(),
    true,
  )
}

pub fn on_anchor_select_ex(req: &Request) -> Response {
  let refs = get_references(req);
  let id = refs[1].to_string();
  let user_dialog = refs.get(2).unwrap_or(&"").to_string();

  let mut m = String::from("\\C");
  m += "\\0\\n\\_q─\\w1──\\w1───\\w1───────\\w1────\\w1──\\w1──\\w1─\\w1─\\n\\_w[750]\\_q";
  if !user_dialog.is_empty() {
    m += &format!("\\1『{}』\\_w[500]", user_dialog);
  }
  match id.as_str() {
    "Fastened" => {
      m += "\
              h1111205文字通りの意味よ。\\n\
              私はこの街から出られない。物理的にね。\\n\
              h1111209この身体は霧とともにあり、霧はこの街にのみ留まる。\\n\
              h1111205この霧との因果を断たないかぎり、私はずっとこの街に閉じこめられたままよ。\
              ";
    }
    "Nolonger" => {
      m += "\
              h1111205何一つ変わらない日々。\\n\
              分かりきった明日は、今日とどう違うの？\\n\
            ";
    }
    _ => return new_response_nocontent(),
  }
  return new_response_with_value(m, true);
}
