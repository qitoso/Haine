use crate::events::aitalk::Talk;
use crate::events::common::*;
use rand::seq::SliceRandom;
use shiorust::message::{Response, *};

pub fn on_stick_surface(_req: &Request) -> Response {
  // \1のサーフェスを\0に重ねて固定する
  let stick_surfaces = "\
  \\1\\_w[100]\
  \\![reset,sticky-window]\
  \\![set,alignmenttodesktop,free]\
  \\![move,--X=0,--Y=0,--time=0,--base=0]\
  \\![set,sticky-window,1,0]\
  ";
  new_response_with_value(stick_surfaces.to_string(), false)
}

pub fn on_boot(_req: &Request) -> Response {
  let talks = Talk::from_vec(all_combo(&vec![
    vec!["h1113105\\1今日も、霧が濃い。".to_string()],
    vec!["\
      h1113105……h1113101\\_w[300]h1113201あら。\\n\
      h1111204いらっしゃい、{user_name}。\
      "
    .to_string()],
  ]));
  let v = format!(
    "\\![bind,シルエット,,0]\\![bind,ex,,0]\\![embed,OnStickSurface]{}{}",
    randomize_underwear(),
    choose_one(&talks, false).unwrap(),
  );
  new_response_with_value(v, true)
}

pub fn on_close(_req: &Request) -> Response {
  let talks = Talk::from_vec(all_combo(&vec![
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
  ]));
  new_response_with_value(choose_one(&talks, true).unwrap().text + "\\-", true)
}

pub fn on_first_boot(_req: &Request) -> Response {
  let m = "".to_string() + "\
    h1000000\\1\\b[10]────今夜、私は死にに行く。\
    \\x\
    \\1\\b[10]短い遺書を書き、身支度を終え、真夜中の街道を静かに歩く。\\n\
    行き先は決めていた。死に場所ぐらい、自分の意志で選びたかったから。\\n\
    夜行バスを捕まえ、乗り込む。\\n\
    \\n\
    カンテルベリオ。その辺鄙な都市の、さらに郊外に位置する古い廃墟が終着点だ。\\n\
    元々廃墟は好きだった。かつての気配を残しながら、忘れ去られてしまった物たちがひっそりとたたずむ姿を美しいと思った。\\n\
    そして、自分もその中で朽ちていけたらと。\\n\
    自分から行動を起こそうと思ったのはいつ以来だろうか。……いや、もうどうでもいい。はやく終わらせたい。\
    \\x\
    \\1\\b[10]…………。\\n\
    しばらく揺られるうちに眠っていたらしい。気づけば目的地に着いていた。\\n\
    荷物をまとめ、停車とともにバスを降りる。\\n\
    いよいよだ。\\n\
    \\n\
    「カンテルベリオの幽霊屋敷」は、\\n\
    その手のマニアの間では有名な廃墟だ。\\n\
    古い洋館で、周囲は小さな庭園に囲まれている。\\n\
    何度か取り壊しの話が出たが、原因不明の事故が続いたため、現在も残っているいわくつきの場所。\\n\
    \\n\
    ここには幽霊が出るという。\\n\
    目撃情報は多々あり、そして、見た者の多くが病気や事故に遭っている。\\n\
    それは呪いによるものなのか、それともただの偶然か。\\n\
    何にせよ、私には関係ない。憧れたもののもとで、私は死ぬ。\
    \\x\
    \\1\\b[10]霧の濃い日だ。カンテルベリオではままあることらしい。\\n\
    数メートル先も見通せないほどの霧だが、不思議と濡れた感じがしない。\\n\
    むしろ乾ききっているような気がして、どことなく不安な気分になる。\\n\
    ……廃墟を目の前にして萎縮しているのだろうか。\\n\
    いや、ここまで来たのだから、引き返すわけにはいかない。\\n\
    私は歩き出した。\
    \\x\
    \\1\\b[10]廃墟の門は錆びつき、固く閉ざされていた。\\n\
    塀の上には、鉄の棘が生えている。写真で確認していた通りだ。\\n\
    荒い麻布を掛けて、その上を乗り越える。\\n\
    運動など全くしない私はそれだけで息が上がってしまった。\\n\
    \\n\
    息を整え、改めてその館を見上げる。\\n\
    その姿は、威圧的で、美しい。\\n\
    ……だが、まごうことなき不法侵入だ。\\n\
    憧れの場所で、生まれて初めて罪を犯した。\\n\
    興奮と、罪悪感があった。\\n\
    \\x\
    \\1\\b[10]玄関の重い扉を開けると、中は暗かった。\\n\
    埃っぽさは無いが、しかし生活感もない。\\n\
    窓をきっちりと覆っている重たげなカーテンは、長いあいだ開いていないようだった。\\n\
    ────その時。\
    \\n\
    \\0\\f[size,20]\\f[color,255,0,0]\\f[italic,true]我らの殿堂に新たなる客人来たれり！\\n\
    汝、己を客なりと信じるならば\\n\
    進み給え。\\n\
    賊なりと念じるならば退き給え。\\f[default]\\n\
    \\x\
    \\1突然の大音声だった。割れ鐘のような声が数人分重なり、広いホールに響き渡る。\\n\
    ……反響が消えると、それ以上の音はまったく聞こえなくなった。\\n\
    闇の中、人の気配はない。\\n\
    \\n\
    鋭い耳の痛みが、幻聴などではないことを示していた。\\n\
    なんだ、これは。\\n\
    いたずらかと思おうとしたが、今の声は……\\n\
    まともな人間の声ではなかった。\\n\
    \\n\
    …………。\\n\
    進むか、退くか。\\n\
    退けば、二度目はないだろう。\\n\
    ……今さら引き返すわけにはいかない。\\n\
    \\x\
    \\1\\b[10]一歩を踏み出すと、\\n\
    \\0\\f[size,20]\\f[color,255,0,0]\\f[italic,true]汝、進みたるを選ぶならば、われらが主と見(まみ)えよ。\\n\
    わが主よ、\\n\
    是を恩と為すか、仇と為すか、\\n\
    その賢明なる眼差しにて、\\n\
    定め給え！\\f[default]\\n\
    \\1耳は痛むが、今度は驚かなかった。代わりに湧き上がるのは強烈な不安。\\n\
    これは、この館は、\"本物\"だ。くだらないいたずらなどではない。\\n\
    ここは怪物かなにかの住処で、私はその主と会わなければならなくなった。\\n\
    主……。もはや決断を違えて逃げることはできないだろう。\\n\
    ……だが案外、正直に事情を話せば、見逃してもらえるかもしれない。\\n\
    無断で入り込んだのは事実だが、盗みなどが目的ではないのだから。\\n\
    ……主というのが、おとぎ話の化け物のようなものだったらどうしようか？\\n\
    引き裂かれて殺されるだろうか。……それはそれで、楽なのかもしれない。\\n\
    どちらにせよ、進まなければ。\
    \\x\
    \\1しばらく歩き、階段を登り、客間へと入ったときだった。\\n\
    人の気配。\\_w[1000]\\n\
    部屋の中央、向かい合ったソファの手前側に座る人影があった。\\n\
    『(いた……！)』\\n\
    人影は静かに立ち上がり、こちらを向いた。\\n\
    \
    \\0\\![bind,シルエット,黒塗り1,1]h1111201\\_w[1000]\
    ────また一人、新しいお客様。\\n\
    \
    \\![bind,シルエット,黒塗り2,1]\
    腕に麻の欠片がついているわ。\\n\
    \\![anim,offset,510002,0,-100]\\n\
    夜を狙って、こっそりと忍び込んだのね。\\n\
    \\![anim,offset,510002,0,-150]\
    そうして、私のもとに。\\n\
    \\![anim,offset,510002,0,-250]\
    そうまでして得たいものがあるのかしら？\\n\
    \
    \\![bind,シルエット,黒塗り4,1]\
    h1111201好奇とは尊く、そして卑しいものだわ。\\n\
    そう思わない？\\n\\n\
    \
    \\1かすかな衣擦れの音とともに暗がりから現れた人影は、少女のようだった。\\n\
    これが主……？\\n\
    僅かに安堵、そして拍子抜けした気持ちは、すぐに不安に塗り潰された。\\n\
    人影というにはひどく違和感がある。\\n\
    暗がりで妖しく光る真っ赤な瞳、死人のように血の気のない肌。\\n\
    人の形をしてはいる。しかし、だからこそ明らかにわかってしまう。目の前の少女は……。\\n\\n\
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
    \\1幽霊……まさか本当に……。\\n\\n\
    \
    h1111204目の前にいるのだから、あなたは信じるしかないのよ。\\n\
    h1111309「カンテルベリオの幽霊」、おおかたあの噂を聞いてきたのでしょう。\\n\\n\
    h1112304おめでとう、私が件の幽霊、そしてこの館の主よ。\\n\
    \\x\
    \
    ……h1111209さて、ここを訪れるほどのことだから、あなたは私たちに興味があるのでしょうね。\\n\
    オカルトフリークが廃墟探索、珍しいことじゃないわ。\\n\\n\
    \\1……微妙に違っていたが、少女は有無を言わせぬ口調で続ける。\\n\
    h1111304あなたたちにとって、理外の存在は魅力的だものね？\\n\
    h1111206その自由さ、存在の不思議さ、\\n\
    あるいは、h1111306死そのものへのあこがれ。\\n\
    あなたは少なからず、それらを抱いていた。\\n\
    h1111209だからあなたはここに来たのでしょう。\\n\\n\
    \\1……それは、当たらずとも遠からず。私がこの場所に惹かれたのは確かだ。\
    h1111209……h1111109……h1111106……。\\n\
    h1121306ふん、あなたのような輩はいつもそう。\
    \\x\
    \
    h1111301…………ねえ、少し頭を使えばわかるはずよ。\\n\
    幽霊がいたとして、\\n\
    自分の住処を悪戯半分で踏み荒らされたとして、\\n\
    それを許すわけがないでしょう？\\n\
    あなた達はいつも無視している。この、私の気持ちを。\\n\\n\
    \\![bind,シルエット,黒塗り2,1]\\![bind,ex,こわいかお,1]\\f[bold,1]\
    " + &shake("あなたは、考えなかったの？") + "\\f[bold,0]\
    \
    \\1一瞬にして少女の雰囲気が変わった。\\n\
    理解するより先に、身体ががたがた震え出す。\
    \\x\
    \\1怖い。怖い。頭の中からそれ意外の思考が消える。\\n\
    事情をわかってもらおうなどと、馬鹿げたことだった。\\n\
    これが、こんな存在が現実にいたなんて。\\n\
    空気がどろりと濁って見えるほどの、凄まじい害意。\\n\
    \
    h1111201\\f[bold,1]……怖い？怖いのでしょうね。\\n\
    もう忘れてしまった感情だわ。至極、どうでもいい。\\n\
    \\n\
    h1111206ふざけた輩には病を憑けて帰しているのだけど、\\n\
    h1111204あなたは……h1111309ふふ、どうしてあげようかしら。\\f[bold,0]\
    \\x\
    \\1……恐怖は依然として消えない。\\n\
    だが、だからこそだろうか。最初の驚きが過ぎていくにつれ、ふと、怒りが込み上げてきた。\\n\
    自分勝手極まりないことだが、頭を覆いつつあるそれは自分でも意外なほど熱く身の内を焦がしていく。\\n\
    \\n\
    ここにはすべてを捨てに来たのだ。なのに、わけのわからない存在にそれを阻まれようとしている。\\n\
    確かに、私はここに立ち入った。だが、それは面白半分などではない。私なりの覚悟があったというのに。\\n\
    そうだ。いつも自分の想いはなにかに邪魔され、否定される。いつも、いつも、いつも。\\n\
    人生最後の望みすら、否定されるのか。\\n\
    じゃあ、私はどうすればいいのだ。\\n\
    どうすれば、よかったのだ。\\n\
    \\n\
    『…………お願いだから、死なせて。』\\_w[1000]\\x[noclear]\\n\
    \\n\
    呟くように口をついで出たのは、今ここで言うつもりのなかった言葉。\\n\
    いつも心のなかに抱きながら、ついに打ち明けることのできなかった言葉。\\n\
    \\x
    h1111109\\![bind,ex,こわいかお,0]\\c…………。h1113207\\![bind,シルエット,,0]くく、ふふふ。\\n\
    h1123201ああ、おかしい。h1123204あなた、ここに死にに来たの？\\n\
    \
    \\1……空気が、緩んだ……？\\n\\n\
    \
    h1113306……こういう方向から来るのは初めてね。\\n\
    h1113201いいわ、h1113204気に入った。\
    \
    \\1……助かった、のだろうか。\\n\
    \\x\
    \
    h1111204命は取らないでおいてあげるわ。\\n\
    h1111206それとも、あなたはそれがお望みだったかしら？\\n\
    h1111209なんにせよ、それは止めよ。\\n\
    h1111201その代わりにあなた、しばらくここに居なさい。\\n\\n\
    \
    \\1φ……？\\n\\n\
    \
    h1111201土足で踏み込んだ報いを与えなければ。\\n\
    お咎めなしでは禍根が残るでしょう。\\n\
    h1111206……必要とはいえ、面倒な誓いを立てたものだわ。\\n\
    h1111209まあ、今回はある意味では丁度いいわね。\\n\
    \
    \\1……私になにをしろと言うのだろう。\
    \\x\
    \
    h1111206……私はね。ここでずっと、待っているのよ。\\n\
    待つことは嫌いではないけれど、話し相手がいなくて退屈していたところだったの。\\n\
    h1111207だから、お話をしましょう。\\n\\n\
    h1111201私はハイネ。カンテルベリオの幽霊。\\n\
    h1111209訪問者\\f[bold,1]{user_name}\\f[bold,0]、この家の主として、あなたの訪問を歓迎します。\\n\
    h1111204せいぜい楽しませてちょうだいね？\\n\\nh1000000\
    \
    \\1いつの間に名前を……。\\n\
    しかし、口を挟む余地はない。\\n\
    客間の椅子を指さすハイネの手。\\n\
    席につけということだ。……従うしかない。\\n\
    当然だが時間はある。捨てるほどに。\\n\
    \\n\
    テーブルを挟み、向かい合って座る。\\n\
    \\n\
    \\0h1111207協力的で結構なことね。\\n\
    h1111201ところで{user_name}、\\n\
    h1111204幽霊の淹れるお茶を飲んだことはあるかしら？h1111207\
    \\1彼女の微笑みは謎めいて親しげだった。\
    ";

  new_response_with_value(m, true)
}

pub fn on_vanish_selected(_req: &Request) -> Response {
  new_response_nocontent()
}

fn shake(text: &str) -> String {
  let mut s = String::new();
  let shakes = [(10, 10), (-14, -14), (4, 4)];
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

fn randomize_underwear() -> String {
  let mut rng = rand::thread_rng();
  let candidates = ["A", "B"];
  format!(
    "\\0\\![bind,下着,{},1]",
    candidates.choose(&mut rng).unwrap()
  )
}
