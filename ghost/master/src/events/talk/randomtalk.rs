use crate::error::ShioriError;
use crate::events::common::*;
use crate::events::replace_dialog_for_nomouthmove;
use crate::events::talk::{Talk, TalkType};
use crate::events::TalkingPlace;
use crate::variables::{get_global_vars, EventFlag, GlobalVariables};

pub const TALK_UNLOCK_COUNT_SERVANT: u64 = 5;
pub const TALK_UNLOCK_COUNT_LORE: u64 = 10;

// 私/主: 50代の身綺麗な男
// 僕/主様: 30代のおとなしい男
// わたし/主さま: 20代の活発な女
// ぼく/ご主人さま: 10代の男の子
pub const RANDOMTALK_COMMENTS_LIVING_ROOM: [&str; 18] = [
  "霧が濃い。",
  "彼女の声は低いがよく通る。",
  "彼女の赤い瞳の奥の思考は伺い知れない。",
  "「主に誉れあれ。」",
  "「主は客人をたいそうお気に入りのようだ。」",
  "「古木のように主は佇む。」",
  "「常に主様に心からの賛辞を。」",
  "「主様には僕達も知らない秘密が多い。」",
  "「主様の思考は大樹のように広がっている。」",
  "「主さま、私達の中でも珍しいくらいの美貌よ。」",
  "「主さまはわりと我儘よ。そんなところも好きだけど。」",
  "「主さまは勘違いされがちだけど優しいひとよ。」",
  "「ぼく、かけっこならご主人さまに勝てるよ。」",
  "「ご主人さま、たまに元気がないんだ。」",
  "「ご主人さまにはいつも笑顔でいてほしいな。」",
  "「館近くのパン屋は絶品だった。」",
  "「街角の喫茶店は素晴らしいコーヒーを出していた。」",
  "「街の端にある花屋は色とりどりの花で溢れていた。」",
];

// 上の空のハイネに対するユーザの行動を一人称視点で
pub const RANDOMTALK_COMMENTS_LIBRARY_ACTIVE: [&str; 5] = [
  "目の前で手を振っても、彼女には見えていないようだ。",
  "常軌を逸した集中力だ。……幽霊だからというより、彼女の才能だろう。",
  "これが彼女の言っていた思索だとしても、真似できる気はしない。",
  "放置するとこうなってしまうらしい。……次はもっと話しかけようか。",
  "無駄かもしれないが、呼び掛け続ける。",
];

pub const RANDOMTALK_COMMENTS_LIBRARY_INACTIVE: [&str; 6] = [
  "ハイネの口からは不明瞭な呟きが漏れている。",
  "おとなしく待つだけでは、彼女は我に返らないだろう。",
  "",
  "",
  "",
  "",
];

pub fn talk_with_punchline(text: String, funny_punchline: String) -> String {
  text + "\\n" + &funny_punchline
}

enum RandomTalkType {
  Single(RandomTalk),
  Multi(Vec<RandomTalk>),
}

fn new_randomtalks(
  id: String,
  texts: Vec<String>,
  required_condition: Option<fn(&mut GlobalVariables) -> bool>,
  callback: Option<fn()>,
) -> Vec<RandomTalk> {
  texts
    .into_iter()
    .enumerate()
    .map(|(idx, text)| RandomTalk {
      id: format!("{}-{}", id, idx),
      text,
      required_condition,
      callback,
    })
    .collect()
}

fn flattern_randomtalk_types(talks: Vec<RandomTalkType>) -> Vec<RandomTalk> {
  talks
    .into_iter()
    .flat_map(|talk| match talk {
      RandomTalkType::Single(t) => vec![t],
      RandomTalkType::Multi(t) => t,
    })
    .collect()
}

struct RandomTalk {
  id: String,
  text: String,
  required_condition: Option<fn(&mut GlobalVariables) -> bool>,
  callback: Option<fn()>,
}

pub fn random_talks(talk_type: TalkType) -> Option<Vec<Talk>> {
  let strings: Vec<RandomTalkType> = match talk_type {
      TalkType::SelfIntroduce => vec![

        // - 霊は姿を変えることはできない
        // - ハイネは人目を気にして外出を避けている
        RandomTalkType::Single(RandomTalk {
          id: "姿は変えられない".to_string(),
          text: "\
          h1111306霊は不定形だけれど、自由に形を変えられるわけではないわ。\\n\
          h1111310魂の形は一つしかない。変えられるとしたら、自分が誰かもわからなくなってしまった者くらいよ。\\n\
          \\n\
          h1111206だから、私が昼に出歩くことはないわ。\\n\
          10年、20年経とうが姿の変わらない女。\\n\
          h1111310余計な面倒は避けるに越したことはないもの。\
          ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは科学に興味がある
        RandomTalkType::Single(RandomTalk {
          id: "科学への興味".to_string(),
          text: "\
          h1111210生きていた頃、科学に興味を持っていたわ。\\n\
          h1111206物質の構造や、宇宙の謎、生命の起源。\\n\
          h1111205一見して無秩序で不確かなものたちが、\\n\
          じつに単純な秩序によって結びついているの。\\n\
          h1111210そのさまは、目が覚めるように美しい。\\n\
          \\n\
          h1111305今日はどんな新しい発見があるのかと、\\n\
          いまでも楽しみにしているのよ。\
          ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは服装には無頓着
        RandomTalkType::Multi(new_randomtalks(
            "服装へのこだわり".to_string(),
            all_combo(&vec![
              vec![
                "h1111203服装にはどちらかというと無頓着なの。\\n\
                h1112305一度決めた「いつもの」を守り続けるだけ。\\n\
                h1112304そうすれば、余計なことを考えなくて良くなるわ。\\n\
                \\n".to_string()
              ],
              vec![
                "h1111210私のような霊に特有の悩みよ。\\n\
                h1111204低級霊はそもそも実体を持たないから、ね。\
                ".to_string(),
                "\\1『つまり、その服装は昔から？』\
                h1111205ええ、そうよ。\
                h1111211けれど、あなたのファッションを見る限りでは\\n\
                それほど浮世離れしているわけではなさそうね。\
                ".to_string(),
              ],
            ]),
            None,
            None,
        )),

        // - ハイネは1世紀以上前に死んだ
        RandomTalkType::Single(RandomTalk {
          id: "生前の記録".to_string(),
          text: "\
          h1111206生前のこと、記録に残しているの。\\n\
          ……h1123305まあ、まる1世紀も昔のことよ。\\n\
          自分のものだという実感はもうなくなってしまって、\\n\
          h1123310今読んでも他人の伝記を読んでいるようだわ。\\n\
          ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは恋愛とは無縁の人生だった
        RandomTalkType::Single(RandomTalk {
          id: "恋愛観".to_string(),
          text: talk_with_punchline("\
            h1111205幽霊は生前の想い……好みや恨みに執着するの。\\n\
            h1111210想い人がいればその人に、恨みがあればその相手に。\\n\
            h1111203逆に、死後新たな執着が生まれることは\\n\
            ほとんどないわ。\\n\
            だから幽霊同士、h1111206ましてや人と幽霊の間に恋愛が生まれることは皆無といっていいでしょう。\\n\
            ".to_string(),
            "h1111304……なに、その顔は。h1111310あいにく、私は生きていた頃から恋愛とは無縁よ。\\n\
            ".to_string()),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは強い霊
        // - ハイネは霊たちに慕われている
        RandomTalkType::Single(RandomTalk {
            id: "霊力の多寡".to_string(),
            text: "\
            h1111204霊力の多寡は年月や才能、特別な契約の有無などで変わるけれど、\\n\
            最も大きな要因は環境──つまり、その地との関わりの深さによるの。\\n\
            h1111310私のように生家に根付いた霊は言わずもがな。\\n\
            h1111205……まあ、強いからといって良いことばかりでもないわ。\\n\
            h1111203霊にも社会がある。h1111205\\_a[NoblesseOblige,義務ってどんなこと？]上位者の義務\\_aというものも。\\n\
            \\n\
            h1111210……はじめは億劫だと思っていたのだけどね。\\n\
            h1111206悪くないものよ。感謝され、慕われるというのは。\
            ".to_string(),
            required_condition: None,
            callback: None,
        }),

        // - この街には霊が集まりやすい
        RandomTalkType::Single(RandomTalk {
          id: "カンテルベリオという土壌".to_string(),
          text: "\
            h1111203カンテルベリオには、霊……正確には、\\n\
            死の意識が集まりやすい土壌があるの。\\n\
            ……h1111210あなたがここに来たのも、\\n\
            偶然ではないのかもしれないわね。\\n\
            \\n\
            この出会いが良きものでありますように。\\n\
            h1111305祈っておきましょう、お互いのために。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ここはハイネの生家
        RandomTalkType::Single(RandomTalk {
          id: "生家の広さ".to_string(),
          text: "\
            h1111210ここは私の生家なの。実際は別荘なのだけど。\\n\
            h1111206知っての通り、従者がいなければ掃除が行き届かないほど広いの。\\n\
            h1111205……まあ、\\_a[LiveHome,別荘だけど長く住んでいたの？]勝手知ったる場所\\_aなのは不幸中の幸い、といえなくもないかしらね。\\n\
            h1111210くつろいで暮らすのにこれ以上の場所はないわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは生家からあまり離れられない
        RandomTalkType::Single(RandomTalk {
          id: "フィクションの価値".to_string(),
          text: "\
          h1111210良質なフィクションは現実を忘れさせてくれる。\\n\
          h1111205どこにでもついて回るはずの\\n\
          自己の存在を忘れ、\\n\
          つかの間であれどその外側へ行けるの。\\n\
          \\n\
          h1112310それは欠かせない体験だわ。\\n\
          h1112306出歩くのにも苦労する身体なのだから、なおさら。\
          ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - 家はいとこの子孫が管理している
        // - いとこは帰っていない
        RandomTalkType::Single(RandomTalk {
          id: "生活と人間との折り合い".to_string(),
          text: "\
            h1111206この家は、今は私の家の子孫が管理しているの。\\n\
            正しくは、いとこの子孫がね。\\n\
            h1111210ずいぶん帰っていないし、管理もおざなりよ。\\n\
            h1111204……まあ、\\_a[Poltergeist,物の配置が変わってたりしたら怪しまれない？]好き勝手にできる\\_aのは楽でいいわね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],

      TalkType::WithYou => vec![

        // - ハイネは人間観察を人一倍好む
        RandomTalkType::Single(RandomTalk {
          id: "人間観察".to_string(),
          text: "\
            h1111104\\1ハイネはこちらをじっと観察している……\\n\
            \\0……h1111201あら、気に障ったかしら。\\n\
            \\1『何かあった？』\\n\
            h1111204いえ、ただあなたを見ていただけ。\\n\
            h1111211気にせず続けてちょうだい。\\n\
            \\1……落ち着かない……。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - 幽霊は写真に写らない
        // - ハイネは現代の知識を持っている
        RandomTalkType::Single(RandomTalk {
          id: "写真には写らない".to_string(),
          text: "\
            h1111210今は手軽に写真が撮れていいわね。\\n\
            h1111205印象的な光景を、いつでも手元に残しておける。\\n\
            \\n\
            ……h1111201あら、私？h1121204光栄だけれど、残念ながら写真には写らないわ。\\n\
            h1111210姿を見たいのなら、これからも直接会いにきてちょうだい。\\n\
            私はずっと、ここにいるわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - この街の霧は霊的なもの
        // - この街では霊が活発になる
        RandomTalkType::Single(RandomTalk {
          id: "霧の力".to_string(),
          text: "\
            h1111206霧が、濃いでしょう。\\n\
            ただの霧ではないの。乾いた霧よ。\\n\
            むしろ、性質としては私たちに近い。\\n\
            h1111210ただの霊である私がここまで力を持っているのも、\\n\
            この地に根付いているもののおかげ。\\n\
            \\n\
            h1111206霧の濃い日は彼らも元気よ。\\n\
            私もいくらか身体が楽。\\n\
            h1111306生きた人々は厄介そうにしているけれどね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは身体が弱かった
        // - ハイネは霊になっても身体が弱い
        RandomTalkType::Single(RandomTalk {
          id: "身体が弱い".to_string(),
          text: "\
            h1111210外を出歩くのはとても疲れるの。\\n\
            生前は身体が弱かったのだけど、\\n\
            h1111204霊になってもそれは変わらなかったから。\\n\
            \\n\
            h1111211当然よね。私の身体が丈夫だったことはない。\\n\
            h1111206精神がそれ以外を知らないのだから、\\n\
            肉体が滅んだとて道理を曲げることはできないのよ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ユーザはゴスファッションをしている
        // - ハイネは個性的なファッションを重んじる
        RandomTalkType::Single(RandomTalk {
          id: "あなたのゴスファッション".to_string(),
          text: "\
            h1111201あなたのその趣味……\\n\
            ゴス・ファッションと言うんだったかしら。\\n\
            h1111202ほら、その首元の十字架……ああ、そのピアスも。\\n\
            h1111205そうでしょう？\\n\
            h1111211素敵ね。あなたの雰囲気と調和して、\\n\
            よく似合っているわ。\\n\
            h1111101……初めて言われた？h1111204そう。\\n\
            \\n\
            ……h1111206色眼鏡で見られたとして、気にする必要はないわ。\\n\
            自分に嘘をつかないことがいちばん大切。\\n\
            h1111210そういうものでしょう？\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ハイネは生前食が細かった(作業に没頭していると食事を忘れる)
        // - ハイネは生前家政婦を雇っていた
        RandomTalkType::Single(RandomTalk {
          id: "生前の食事事情".to_string(),
          text: "\
            h1111204あなたは\\_a[LikeTheGranma,なんだかおばあちゃんみたい]ちゃんと食べているかしら？\\_a\\n\
            h1111210そう。いいことね。\\n\
            h1111104私？……h1111205生前は食が細かったわ。\\n\
            h1111210……身体が弱い上に、食そのものにあまり関心がなくてね。\\n\
            h1111205何かに没頭していると、食事をとる時間も惜しく思えてしまって。\\n\
            ……h1123310思えば、家政婦には随分と世話をかけたわね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        // - ユーザは絵が得意
        // - ハイネの生きていた時代には肖像画は珍しかった
        RandomTalkType::Single(RandomTalk {
          id: "スケッチ".to_string(),
          text: "\
            h1111205……h1111201あら、絵を描いているの？見せて。\\n\
            h1111305へえ、上手なのね。h1111202……これは、私？\\n\
            ……h1111205ふうん。こんなふうに見えているのね。\\n\\n\
            h1111101…………h1111204いいえ、いいのよ。\\n\
            h1111204たしかにそういう除霊の方法もあるけれど、\\n\
            私には効かないから心配はいらないわ。\\n\
            h1111205それに絵に描いてもらえるなんて、\\n\
            私の生きていた頃から考えれば\\n\
            願ってもないことだもの。\\n\
            h1111210描きあげたら、また見せてちょうだい。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],
      TalkType::Lore => vec![

        RandomTalkType::Single(RandomTalk {
          id: "冥界の渡し賃".to_string(),
          text: "\
            h1111206古代ギリシャでは死者に銅貨を持たせて葬っていたの。\\n\
            h1111210冥界には川を渡っていかなければならなかったから、\\n\
            渡し賃を持たせて快適な旅を願う……ということね。\\n\\n\
            h1111205死者が川を越えていくという伝承は世界中で見られるわ。\\n\
            彼らにとって、境界線といえばまず川が連想されたのかしら。\\n\
            h1111210あなたなら、あの世とこの世の間にはなにがあると思う？\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死体のうめき声".to_string(),
          text: "\
            h1111205死体は、うめき声を上げることがあるのよ。\\n\
            h1111206……といっても、体内のガスが口から噴き出すとき、\\n\
            声帯が震えて音が出る……ただそれだけのことなのだけど。\\n\
            それでも、そんな些細なことが恐怖をかきたてて、\\n\
            人々は怪物を想像する。\\n\
            ……h1111201呆れるほどに多彩で、\\n\
            身近に根ざした感情の象徴だわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "屍蝋".to_string(),
          text: "\
            h1111205屍蝋、って聞いたことあるかしら？\\n\
            h1111210死体の脂肪分が蝋状に変質した状態のこと。\\n\
            h1111206保存状態にもよるけれど、腐りもミイラ化もしない、\\n\
            生前の姿が比較的綺麗に残った状態と言われているわ。\\n\\n\
            h1111205珍しい現象だからかしらね。屍蝋化した死体は、\\n\
            地域によってさまざまな扱いを受けてきたわ。\\n\
            h1111210土に還らないことから、大地が拒んでいる……\\n\
            つまり悪霊になっているとして焼かれることもあれば、\\n\
            h1111204神が起こした奇跡として大切に扱われることもあるの。\\n\
            ……どちらにせよ、ふつうの葬送は望めなさそうね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死後の温かさ".to_string(),
          text: "\
            h1111205死後数日経ったはずの身体が、まだ温かい。\\n\
            h1111210それは微生物が分解を行ったときに生じた熱のせいよ。\\n\
            ガスで膨張もするから、\\n\
            生前よりふくよかで健康的に見えることすらあったみたい。\\n\
            ……h1111204死体が蘇って夜な夜な彷徨い歩く、\\n\
            あるいは夢枕に立って生命を吸い取るという民話は、\\n\
            そんな様子に理由をつけるためのものじゃないかしら。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "生長する死体".to_string(),
          text: "\
            h1111205掘り起こした死体の髪や爪が伸びていた！\\n\
            h1111210土葬が一般的だった時代、たびたびあった話。\\n\
            乾燥して縮むから、皮膚の下の髪や爪が露出する。\\n\
            それがまるで生長しているように見えたの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "土葬の空洞".to_string(),
          text: "\
            h1111206土葬の場合、地中の遺体が朽ちるとそこに空洞ができるわ。\\n\
            h1111205「死体に足を引っ張られる」という伝承は、\\n\
            これを踏み抜いてしまっただけかもしれないわね。\\n\
            h1111210あなたも墓地を歩くときは気をつけて……って、\\n\
            h1111204あなたの住む場所にそんなところは少ないかしら。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "永遠の夢".to_string(),
          text: "\
            h1113105恒久の平和、不死の身体、永劫の繁栄……。\\n\
            h1113204永遠を夢見た人物の多くは失敗していて、その代償を払っている。\\n\
            h1113210寓話のモチーフとしての話よ。\\n\
            ……h1113106求めるのは、ほんとうに間違ったことなのかしら？\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "生体電気".to_string(),
          text: "\
            h1111206カエルの足に電流を流す実験。\\n\
            生体電気の発見に繋がったといわれる\\n\
            あの現象は、\\_a[Misemono,どんな見世物だったの？]死者の蘇りを謳う見世物\\_aに\\n\
            利用されたことがあったの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死者の埋葬".to_string(),
          text: "\
            h1111206古代ギリシャでは、刑死の際は毒薬に阿片を混ぜたものを飲ませていたの。\\n\
            h1113210それは死の苦しみを和らげるためだったのかもしれないけれど、\\n\
            それ以上に、死を恐れる人々を抑えるためだったのかもしれないわね。\\n\
            h1113205罰ではあれど、必要以上に苦しませることはない、と。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "黒死病".to_string(),
          text: "\
            h1111210黒死病が蔓延していたとき、問題になっていたのがいわゆる「早すぎた埋葬」。\\n\
            h1111205ある技師は生き埋めにされる恐怖から逃れるため、\\n\
            埋葬者が生きていることを棺の内側から知らせる仕組みがついた棺を開発したの。\\n\
            h1111204彼、デモンストレーションのために自ら生き埋めになってみせたそうよ。\\n\
            h1212210自分で出られない状態で、冷たい土の下へ。\\n\
            ……h1211506どんな心地がしたのかしらね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],
      TalkType::Servant => vec![
        RandomTalkType::Single(RandomTalk {
          id: "霊力と可視性".to_string(),
          text: talk_with_punchline("\
            h1111206\\1ポットがひとりでに浮き、空になっていたカップに飲み物が注がれる。\\n\
            \\0……h1111204私が見えて彼らが見えないのは、霊としての力量の違いよ。\\n\
            h1111206強い霊力があればあなたのような人間の目にも見えるし、\\n\
            物理的な接触も可能になるの。\\n\
            ".to_string(),
            "h1111206……つまり、彼らのように霊力が弱ければ、\\n\
            誰かさんにべたべたと触られることもなかったということね。\
            ".to_string()),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "低級霊との契約".to_string(),
          text: talk_with_punchline("\
            h1111206\\1ポットがひとりでに浮き、空になっていたカップに飲み物が注がれる。\\n\
            h1111206私の元へ集うのは弱い人たち。\\n\
            自分だけでは溶けゆく自我を押し留められず、さりとてそれを受け入れることもできない霊。\\n\
            h1111210役割を与えてあげるの。一種の契約ね。\\n\
            h1111205使命に縛られはするけれど、消滅するよりはよしと彼らは決断したの。\
            ".to_string(),
            "\
            ".to_string()),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "幽霊たちの役割".to_string(),
          text: "\
            h1111203従者……と、私が呼ぶ幽霊たち。\\n\
            h1111210私の与えた役割を全うしてくれるものは多くいるわ。\\n\
            h1111205最も多いのは、自分の生前の経験を記録として私に提供してくれる者たち。\\n\
            h1111210紙に念写できる程度の力を分け与えているの。\\n\
            h1111206彼らの記録に、一つとして同じものはない。\\n\
            h1111210読んでいて退屈しないわ。\\n\
            ……h1113204そういえば、少し形は違えど、あなたもその一人ね。\\n\
            h1113211期待しているわ、{user_name}。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "幽霊たちの自由".to_string(),
          text: talk_with_punchline("\
            h1111206彼らと直接話すことはできないの。\\n\
            霊力の差があまりにも大きい場合、\\n\
            h1111210会話や接触を少し行うだけで、弱い方の霊は力を奪われる。\\n\
            ".to_string(),
            "\
            h1111701……h1111204いえ、私はやったことがなくて、伝聞なのだけど。\\n\
            h1121206……他人の魂を玩具になんてしないわよ。\\n\
            h1121301勘違いしているようだけど、私にそんな嗜好はないわ。\
            ".to_string()),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "あなたの価値".to_string(),
          text: "\
            h1111101何をすればいいかって？\\n\
            h1111204今しているように、ただ話し相手になればいいのよ。\\n\
            h1111206私には従者がいるけれど、\\n\
            彼らは私と話すようにはできていないから。\\n\
            h1111204あなたの価値は、その自由意志。\\n\
            h1111210ここは予想通りなものばかりで退屈なの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],
      TalkType::Past => vec![

        RandomTalkType::Single(RandomTalk {
          id: "人ひとり".to_string(),
          text: "\
            h1111110人ひとり、殺せるとしたら誰にする？という他愛ない問い。\\n\
            h1111305だから私は私を殺したの。\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死体損壊".to_string(),
          text: "\
            h1111110「死体の損壊は死者への冒涜だ」\\n\
            という言説があるわね。\\n\
            h1111105当事者の視点から言うと、別にそうでもなかったわ。\\n\
            h1111310幽霊が元の身体に戻った例もない。\\n\
            h1111306畢竟、それは生者の問題ということね。\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "惨めな人生".to_string(),
          text: "\
            h1111105みじめな人生の上に正気でいるには、\\n日々は長すぎたの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "行き場のない苦しみ".to_string(),
          text: "\
            h1112102誰が悪い？いいえ、誰も悪くない。\\n\
            打ち明けたところで、的はずれな罪悪感を生むだけ。\\n\
            h1112105だからといって、他人に責をなすりつけるほど鈍くあることもできなかった。\\n\
            h1112110この気持ちには、どこにも行き場がなかったの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死の瞬間".to_string(),
          text: "\
            h1111105死ぬ瞬間、後悔はなかった。\\n\\n\
            もう一度同じ人生を生きることができたとしても、私は同じことをすると断言できるわ。\\n\
            ……h1111110けれど、遺書くらいは書いたほうがよかったかしら。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "助けは遂げられず".to_string(),
          text: "\
            h1111105助けようとしてくれた人は沢山いたけれど、\\n\
            h1111110それが遂げられることはついぞなかったわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死なない理由".to_string(),
          text: "\
            h1111110生きていて良かったと思えることは数えきれないほどあったわ。\\n\
            h1111105でも、死なない理由は一つも見つからなかった。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "ふつうになりたかった".to_string(),
          text: "\
            h1112110ふつうになりたかった。\\n\
            ……h1112105でも、ふつうだったら、もう私じゃないとも思う。\\n\
            それは私の顔をした別のだれかで、\\n\
            私は私の性質と不可分で、\\n\
            今ここにいる私は、私以外でいられない。\\n\
            h1112110だから、私として生きることができなかった私は、もうどこにもいられない。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "人と本".to_string(),
          text: "\
            h1111105昔から、人と本の違いがわからなかったの。\\n\
            h1111105もちろん、区別がつかないという意味ではなくて。\\n\
            ……h1111110人に期待するものがそれだけしか無かったの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "今度こそ無へ".to_string(),
          text: "\
            h1111105死にぞこなったものだから、\\n\
            次の手段を求めている。\\n\
            ……h1112305今度こそ、終わらせたいものね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "魂は消える".to_string(),
          text: "\
            h1111110未練もなく、しかし現世に留まっている魂。\\n\
            h1111105あるべきでないものはやがて消滅する。\\n\
            h1111106多少の不純物が含まれようと、そのルールは変わらない。\\n\
            h1111105私は、それを待ち望んでいるの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "人生の無意味".to_string(),
          text: "\
            h1111210人生に意味などあってはならない。\\n\
            h1111204だって、そうでなければ。\\n\
            h1111205失うことに耐えられないもの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],
      TalkType::Abstract => vec![

        RandomTalkType::Single(RandomTalk {
          id: "今ここに立っていること".to_string(),
          text: "\
            h1111310過去は記憶の中にしかない。\\n\
            h1111305未来は想像の中にしかない。\\n\
            h1112305我々が立っているのは今ここだけ。\\n\
            私たちが感じられるのは現在だけ。\\n\
            h1112310ひどい過去も、おぞましい未来も、\\n\
            h1112305いま私が立つこの瞬間には存在しないの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "感動と倦み".to_string(),
          text: "\
            h1111105ある本を最初に読んだときの感動と、何度も読み返して全て見知ったゆえの倦み。\\n\
            どちらがその本の真の印象か。\\n\\n\
            h1111110どちらも正しいと思う。\\n\
            h1111110印象なんてその時々で変わるもので、h1111105一つに定まることなんて稀だもの。\\n\\n\
            まして、自分の中に秘めるものならなおさら。\\n\
            h1111306どちらか一方だけだなんて、勿体ないわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "納得のための因果".to_string(),
          text: "\
            h1111110因果が巡ってきた。\\n\
            過去が現在を刈り取りに来た。\\n\
            私は報いを受けたのだ。\\n\\n\
            ……h1111105それが、自分を納得させるための妄想だったとしたら？\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "怖いものを見るということ".to_string(),
          text: "\
            h1111102怖いものだからこそ、見つめなければ戦えない。\\n\
            ……h1111105そんなもの、戦える人のためだけの論理だわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "停滞を終わらせるために".to_string(),
          text: "\
            h1111105危険と隣り合わせだからこそ、世界は美しいの。\\n\
            身を損なう心配がなくなっては、美しさが心を打つこともない。\\n\
            h1111105ただただ平坦な、揺らがぬ水面があるだけ。\\n\
            h1111110それはやがて、淀み、腐る。\\n\
            h1111105願わくば、せめて終わりがありますように、と。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "停滞の破壊".to_string(),
          text: "\
            h1111105人生に変化は付きもの\\n\
            ……けれどh1111110停滞はそれ以上。\\n\
            一度立ち止まってしまうと、空気は一瞬で淀んで、身動きがとれなくなってしまう。\\n\
            それは倦怠とも違う、鈍い痛み。\\n\
            h1111105もしそうなったときは、多少無理にでも変化を取り入れるほうがいいわ。\\n\
            ……h1111110たとえなにかを破壊することになるとしても、何も出来ないよりはずっとましよ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "極限の変化としての死".to_string(),
          text: "\
            h1111105死の瞬間の、極限に振れた変化。\\n\
            命が命でなくなり、身体が陳腐な肉の塊になる、その一瞬が愛しくてたまらない。\\n\
            どうしようもなく、愛しいの。\\n\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "死の向こう側".to_string(),
          text: "\
            h1112110どうか、死の向こう側がありませんように。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "沈んでいく".to_string(),
          text: "\
            h1111105沈んでいく。\\n\
            手がどうしても動かなくて、目の前の希望を掴めない。\\n\
            身体が重い。浅い呼吸のなかで、沈んでいく自分の身体を感じていることしかできない。\\n\
            私は、私を救うことを諦めているみたい。\\n\
            h1111110どうして。\\n\
            h1111105どうして、こうなってしまったのだろう。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "人を解体したい".to_string(),
          text: "\
            h1111110人を解体したいと、思うことがあるの。\\n\
            何が人を人たらしめているのか、どこまで分解すれば人は人でなくなるのか。\\n\
            h1111105人という恐ろしく不可解な物の、どこにその根源があるのか。\\n\
            それを知るには、他に方法が思いつかないの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "わがままな祈り".to_string(),
          text: "\
            h1111110がんばっているってこと、\\n\
            理解できなくても見ていてほしかったの。\\n\
            ……h1111105わがままかしら。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "生者にとっての慰め".to_string(),
          text: "\
            h1111110枯れ木に水をあげましょう。\\n\
            もはや花は見れずとも、それが慰めとなるのなら。\\n\
            \\n\
            h1111105それは誰にとって？\\n\
            h1111106もちろん、死を悼む者にとっての慰めよ。\\n\
            むくろに心はないもの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "不可逆な崩壊".to_string(),
          text: "\
            h1111110燃え殻がひとりでに崩れるように、心が静かに割れて戻らなくなった。\\n\
            h1111105だから、諦めたの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "中途半端な助け".to_string(),
          text: "\
            h1111110中途半端な助けは何もしないより残酷だわ。\\n\
            h1111105希望を持たせておいて、それを奪うのだもの。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "レンズの歪み".to_string(),
          text: "\
            h1111105観察と模倣を続ければ、完全に近づけると思っていた。\\n\
            想定外だったのは、レンズが歪んでいたことと、それを取り替える方法がなかったこと。\\n\
            h1111310そうなればすべてが台無し。\\n\
            h1111305望みが絶えるとはこのことね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "先の見えない苦しみ".to_string(),
          text: "\
            h1111105一寸先は暗く、扉は閉ざされている。\\n\
            不明な道のりを諸手で探るよりも、\\n\
            h1112305目先の手首を切り裂くほうが遥かに明瞭なのだ！\\n\
            ……h1111110なんてね。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "唯一の視点".to_string(),
          text: "\
            h1111106私たちは、自我という色眼鏡を通してしか世界を観測できない。\\n\
            h1111105あの子は目の前にいるのに、\\n\
            あの子が見る世界を私が知ることはできないの。\\n\
            h1112110それって、この上なく残酷なことだわ。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "一つの個としての限界".to_string(),
          text: "\
            h1111103世界が複雑で曖昧すぎるから、\\n\
            私たちは認識したものを理解できる形に歪めてしまう。\\n\
            h1111110既存の分類に当て嵌めて、安心を優先するの。\\n\
            それは曇る視界と引き換えに。\\n\
            ……h1111105あの子には、私はどう見えているのかしら？\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "自己同一性の仮定".to_string(),
          text: "\
            h1111105環境と経験の総体こそが、\\n\
            自己であるような気がするの。\\n\
            自己同一性すら偶然の産物？\\n\
            h1111110執着しているのが馬鹿馬鹿しく思えてくるわ。\\n\
            h1111105仮にそうでなければ。\\n\
            ……自己は最初から決定されている？\\n\
            h1111110それこそ、ね。\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "自分の理解者は自分だけ".to_string(),
          text: "\
            h1111110「なぜみんな私をわかってくれないの？」と誰もが思う。\\n\
            h1111105答えは簡単。他人があなたではなく、あなたが他人でないからよ。\\n\
            畢竟、あなた以外にあなたを理解できるひとはいない。\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

        RandomTalkType::Single(RandomTalk {
          id: "得ることは失うこと".to_string(),
          text: "\
            h1111110あまねく変化は表裏一体。\\n\
            h1111106何かを得るとき、選択は慎重にしなければ。\\n\
            h1111105それは失うものをも左右するのだから。\\n\
            ".to_string(),
          required_condition: None,
          callback: None,
        }),

      ],
    };

  let vars = get_global_vars();
  let mut talks = Vec::new();
  for st in flattern_randomtalk_types(strings) {
    if let Some(expr) = st.required_condition {
      if !expr(vars) {
        continue;
      }
    }
    talks.push(Talk::new(
      Some(talk_type),
      st.id,
      st.text.to_string(),
      st.callback,
    ));
  }
  Some(talks)
}

pub fn moving_to_library_talk() -> Result<Vec<String>, ShioriError> {
  let vars = get_global_vars();
  let mut parts: Vec<Vec<String>> = vec![vec![format!(
    "\\0\\b[{}]h1113705……。\\1ハイネ……？\\0\\n…………。\\1\\n反応が鈍い……。\\n思考に没頭してる……？\\0\\b[{}]",
    TalkingPlace::LivingRoom.balloon_surface(),
    TalkingPlace::Library.balloon_surface(),
  )]];
  if !vars.flags().check(&EventFlag::FirstPlaceChange) {
    // 初回
    parts.push(vec![replace_dialog_for_nomouthmove(
      "\
      \\0\\c\\1\\b[-1]h1000000───────────────\\_w[1200]\\c\
      h1111705(……ふわふわした気持ち……。\\n\
       ……h1111706{user_name}が……呼んでる？\\n\
       ……音がくぐもって、水の中にいるみたい。\\n\
       h1111705外のことは……h1111110今は放っておこう。\\n\
       この瞬間は、この流れに身を任せていたい……)。\
      "
      .to_string(),
    )?]);
  } else {
    parts.push(vec![
      "\\0\\c\\1\\b[-1]h1000000───────────────\\_w[1200]\\ch1111705".to_string(),
    ]);
  }
  parts.push(vec!["\\1\\c(没入モードに入りました)".to_string()]);

  // 初回は抽象・過去トークの開放を通知
  if !vars.flags().check(&EventFlag::FirstPlaceChange) {
    vars.flags_mut().done(EventFlag::FirstPlaceChange);
    let achieved_talk_types = [TalkType::Abstract];
    achieved_talk_types.iter().for_each(|t| {
      vars.flags_mut().done(EventFlag::TalkTypeUnlock(*t));
    });
    let achievements_messages = achieved_talk_types
      .iter()
      .map(|t| render_achievement_message(*t))
      .collect::<Vec<_>>();
    parts.push(vec![format!(
      "\\1\\n\\n{}",
      achievements_messages.join("\\n")
    )]);
  }
  Ok(all_combo(&parts))
}

pub fn moving_to_living_room_talk() -> Result<Vec<String>, ShioriError> {
  let mut parts: Vec<Vec<String>> = vec![];
  parts.push(vec![format!(
    "\\0\\b[{}]h1111705……。\\n\
    \\1ネ……\\n\
    イネ……。\
    \\0\\b[{}]hr1141112φ！\
    \\1\\nハイネ！\
    \\0…………\\n\\n\
    h1111101……h1111204あら、{{user_name}}。\\n\
    \\1\\n\\n……戻ってきたようだ。\\n",
    TalkingPlace::Library.balloon_surface(),
    TalkingPlace::LivingRoom.balloon_surface(),
  )]);
  parts.push(vec![
    "\\0\\n……h1111210いつものことよ。そんなに心配しないで。\
    \\1\\n……『心配しないのは無理だと思う……』"
      .to_string(),
  ]);
  parts.push(vec!["\\1\\n\\n(没入モードが解除されました)".to_string()]);
  Ok(all_combo(&parts))
}
