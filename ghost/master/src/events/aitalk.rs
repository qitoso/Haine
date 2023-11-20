use crate::events::common::*;
use crate::variables::get_global_vars;
use once_cell::sync::Lazy;
use shiorust::message::{Request, Response};

pub static TALKS: Lazy<Vec<String>> = Lazy::new(|| {
  vec![
    format!("\
    h1111206霧が、濃いでしょう。\\n\
    ただの霧ではないの。むしろ、性質としては私たちに近い。\\n\
    h1111209ただの霊である私がここまで力を持っているのも、\\n\
    この地に根付いているもののおかげ。\\n\\n\
    h1111205次も、霧の濃い日にいらっしゃい。\\n\
    そのほうが、身体が楽なの。\
    "),

    format!("\
    h1111109未練もなく、しかし現世に留まっている魂。\\n\
    h1111105あるべきでないものはやがて消滅する。\\n\
    h1111205私は、それを望んでいるの。
    "),

    format!("\
    h1111205危険と隣り合わせだからこそ、世界は美しいの。\\n\
    身を損なう心配がなくなっては、美しさが心を打つこともない。\\n\
    h1121205ただただ平坦な、揺らがぬ水面があるだけ。\\n\
    h1121209それはやがて、淀み、腐る。\\n\
    h1111205願わくば、せめて終わりがありますように。\
    "),

    "\
    h1111105人生に変化は付きもの……けれどh1111109停滞はそれ以上。\\n\
    一度立ち止まってしまうと、空気は一瞬で淀んで、身動きがとれなくなってしまう。\\n\
    それは倦怠とも違う、鈍い痛み。\\n\
    h1111201あなた。h1111206もしそうなったときは、多少無理にでも変化を取り入れるほうがいいわ。\\n\
    ……h1111209たとえなにかを破壊することになるとしても、何も出来ないよりはずっとましよ。\
    ".to_string(),

    "\
    h1111205死の瞬間の、極限に振れた変化。\\n\
    私は、それがどうしようもなく愛しいの。\\n\
    命が命でなくなり、身体が陳腐な肉の塊になる、その一瞬が愛しくてたまらない。\\n\\n\
    h1111206あなたの目には、贔屓目にも綺麗には写らないのでしょう。\\n\
    h1111309……でもね。倫理は私を救わない。\\n\
    あなたの心を私が握る必要もない。\\n\
    h1111304それを諦められるくらい、私は、これを諦められないのだから。\\n\\n\
    h1111205私はただ、願うだけ。\
    ".to_string(),

    "\
    h1111209あなたたちが歩いている姿を、いつも窓から見ているの。\\n\
    h1111204いつも何かをして、どこかへ向かっている。\\n\
    h1111207羨ましいわ。h1111207私は\\_a[Fastened,どういうこと？]見ていることしかできない\\_aから、なおさら。\
    ".to_string(),

    "\
    h1111105あの人は、私を守ってくれた。\\n\
    でも、私を救えはしなかった。\\n\
    理解と共感は、違う。h1112105……違うのよ。\
    ".to_string(),

    "\
    h1111205私に救いは訪れなかった。\\n\
    想いは、今もずっと、私の中にある。\\n\
    あなたが、私を救える人だったら良かh1111101……。\\n\
    ……h1111109……。いえ、死んだ後で報われようだなんて。\\n\
    h1121205…………h1121305ごめんなさい。\
    ".to_string(),

    "\
    h1122209ふつうになりたかった。\\n\
    ……h1122205でも、ふつうだったら、もう私じゃないとも思う。\\n\
    それは私の顔をした別のだれかで、\\n\
    私は私の性質と不可分で、\\n\
    今ここにいる私は、私以外でいられない。\\n\
    h1122209だから、私として生きることができなかった私は、もう。\
    ".to_string(),

    "\
    h1111209黒死病が蔓延していたとき、問題になっていたのがいわゆる「早すぎた埋葬」。\\n\
    h1111205ある技師は生き埋めにされる恐怖から逃れるため、\\n\
    埋葬者が生きていることを棺の内側から知らせる仕組みがついた棺を開発したの。\\n\
    h1111204彼、デモンストレーションのために自ら生き埋めになってみせたそうよ。\\n\
    h1212209自分で出られない状態で、冷たい土の下へ。\\n\
    ……h1212205どんな心地がしたのかしらね。\
    ".to_string(),

    "\
    h1111205ある本を最初に読んだときの感動と、何度も読み返して全て見知ったゆえの倦み。\\n\
    どちらがその本の真の印象かしら？\\n\\n\
    h1111209私はどちらも正しいと思う。印象なんてその時々で変わるもので、h1111205一つに定まることなんて稀だもの。\\n\\n\
    そう。正反対の感想を同時に抱いたっていいのよ。\
    ".to_string(),

    "\
    h1111201あなたの趣味……ゴスファッション、と言うんだったかしら。\\n\
    h1111207素敵ね。よく似合っているわ。\\n\
    ……h1111206現代ではなにかと色眼鏡で見られるそうだけど、気にする必要はないの。\\n\
    h1111209自分に嘘をつかないことが、いちばん大切だから。\
    ".to_string(),

    format!("\
    h1111202幽霊は外見を変えることもできるの。h1111207……こんなふうに。\\n\\n\
    h1000000{}\\n\
    \\0髪も、身長も、年齢すら、私たちには関係ないの。\\n\
    h1111209\\1瞬きしたとき、彼女はまた元の姿に戻っていた。\
    h1111204あまり大きく変化すると自己認識が揺らいでしまうから、基本的には最も自分らしい姿をすることが多いわ。\\n\
    h1111207こういう戯れもたまにはいいでしょう？\
    ",
    user_talk("……髪が伸びてる！","彼女の姿が揺らいだかと思うと、その姿は一瞬で変わった。", true)),

    "\
    h1111203たまに、街に出ることもあるのよ。\\n\
    私の時間を費やすには、ここの蔵書だけでは足りないから。\\n\
    ……h1111209大丈夫よ。館を出れば、私の存在を感知できる人はいない。\\n\
    h1111205私の存在の根は、どうしたってここにあるの。\
    ".to_string(),

    "\
    h1111209寄る辺ない幽霊はいつか消える。\
    それが10年後なのか、100年後なのか、それとも明日なのか。\\n\
    それは分からないけれど、その日は必ず来る。\\n\
    h1111205だからあなた、いつか消える幽霊にかまけるなんて、時間の無駄なのよ。\\n\
    ……h1111209いつ、来なくなっても。私は気にしないわ。\
    ".to_string(),

    "\
    h1112109どうか、死の向こう側がありませんように。\
    ".to_string(),

    "\
    h1111205みじめな人生の上に正気でいるのに、日々は長すぎたの。\\n\
    ".to_string(),

    "\
    h1111205見慣れたはずの場所にいながら、いつもと違う道に迷いこんだことはある？\\n\
    \\n\
    h1111204もしそうなったら、「\\_a[Yomotsuhegui,ヨモツヘグイって？]ヨモツヘグイ\\_a」……\
    つまり、食べ物には常に注意しなさい。\\n\
    h1111205一度だけなら、は許されない。それがすべてを変えてしまうような落とし穴がこの世にはあるの。\
    ".to_string(),

    "\
    h1111205幽霊は生前の想い……好みや恨みに執着するの。\\n\
    h1111209想い人がいればその人に。恨みがあればその相手に。\\n\
    h1111203逆に、死後新たな執着が生まれることはほとんどないわ。\\n\
    だから幽霊同士、h1111206ましてや人と幽霊の間に恋愛が生まれることは皆無といっていいでしょう。\\n\
    h1111304……なに、その顔は。h1111309あいにく、私は生きていた頃から恋愛とは無縁よ。\\n\
    ".to_string(),

    "\
    h1111105沈んでいく。希望を掴むための手が、どうしても動かない。\\n\
    身体が重い。浅い呼吸のなかで、沈んでいく自分の身体を感じていることしかできない。\\n\
    h1111109どうして、こうなってしまったのだろう。\
    ".to_string(),

    "\
    h1111109人を解体したいと、思うことがあるの。\\n\
    何が人を人たらしめているのか、どこまで分解すれば人は人でなくなるのか。\\n\
    h1111105人という恐ろしく不可解な物の、どこにその根源があるのか。\\n\
    それを知るには、それしかないと思って。\
    ".to_string(),

    "\
    h1111205生前から、人と本の違いがわからなかったの。\\n\
    h1121204もちろん、区別がつかないという意味ではなくて。\\n\
    ……h1111209それだけ、人に期待するものが限られていたということ。\
    ".to_string(),
  ]
});

#[allow(dead_code)]
static BOTSU: Lazy<Vec<String>> = Lazy::new(|| {
  vec![
    format!("\
    h1111205悲しい。ここに縛られていることが、ではない。私が見ることのできない世界のこと。\\n\
    ……h1111209何も、知ることができないの。老人の、幼子の、男の、女の、見る世界を、\\n\
    そのすべてを私がこの身で知ることはかなわない。h1111205決して、……h1111209決して。\\n\
    それが、悲しくて、悔しくて、気が狂いそうになる。\\n\
    h1122305だって、せっかくこの世に生まれたのに。こんなにも自由なのに。……この手で、何をすることもできるはずなのに！\\n\\n\\_w[1200]\
    {}\
    \\0……h1111205あなたに言っても詮無いことだわ。忘れてちょうだい。\
    ",
    user_talk("ハイネ……。","思わず、声が漏れる。", false)),
  ]
});

pub fn version(_req: &Request) -> Response {
  new_response_with_value(String::from(env!("CARGO_PKG_VERSION")), false)
}

pub fn on_ai_talk(_req: &Request) -> Response {
  let vars = get_global_vars();
  vars.volatility.last_random_talk_time = vars.volatility.ghost_up_time;
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
      h1111209私の身体はここに縛られている。\\n\
      h1111205きっと、それは消滅する瞬間まで変わらないでしょう。\\n\
      ";
    }
    _ => return new_response_nocontent(),
  }
  return new_response_with_value(m, true);
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::variables::get_global_vars;
  use shiorust::message::parts::*;
  use shiorust::message::Request;
  use std::collections::HashMap;

  #[test]
  fn test_aitalk() -> Result<(), Box<dyn std::error::Error>> {
    let mut vars = get_global_vars();
    vars.load();
    vars.volatility.idle_seconds = 2;
    vars.volatility.idle_threshold = 1;

    let req = Request {
      method: Method::GET,
      version: Version::V20,
      headers: Headers::new(),
    };
    let mut results = HashMap::new();
    for _i in 0..100 {
      let res = on_ai_talk(&req);
      let value = res.headers.get(&HeaderName::from("Value")).unwrap();
      let md5 = format!("{:x}", md5::compute(value.as_bytes()));
      let n = results.get(&md5).unwrap_or(&0);
      results.insert(md5, n + 1);
    }
    for (k, v) in results.iter() {
      println!("{}: {}", k, v);
    }
    Ok(())
  }
}
