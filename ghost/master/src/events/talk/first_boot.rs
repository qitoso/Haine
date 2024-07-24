use once_cell::sync::Lazy;

pub const FIRST_BOOT_MARKER: &str = "邂逅";

pub static FIRST_BOOT_TALK: Lazy<String> = Lazy::new(|| {
  "".to_string() + "\\t\\*\
    h1000000\\1\\b[10]────今夜、私は死にに行く。\
    \\x\
    \\1\\b[10]短い遺書を書き、身支度を終え、真夜中の街道を静かに歩く。\\n\
    行き先は決めていた。死に場所ぐらい、自分の意志で選びたかったから。\\n\
    夜行バスを捕まえ、乗り込む。\\n\
    \\n\
    カンテルベリオ。その辺鄙な都市の、さらに郊外に位置する古い廃墟が終着点だ。\\n\
    \\n\
    元々廃墟は好きだった。かつての気配を残しながら、忘れ去られてしまった物たちがひっそりとたたずむ姿を美しいと思った。\\n\
    そして、自分もその中で朽ちていけたらと。\\n\
    自分から行動を起こそうと思ったのはいつ以来だろうか。……いや、もうどうでもいい。すべて終わるのだから。\
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
    \\x\
    \\1しばらく歩き、階段を登り、客間へと入ったときだった。\\n\
    人の気配。\\_w[1000]\\n\
    部屋の中央、向かい合ったソファの手前側に座る人影があった。\\n\
    『(いた……！)』\\n\
    人影は静かに立ち上がり、こちらを向いた。\\n\
    \
    \\0\\![bind,シルエット,黒塗り1,1]h1111201\\_w[1000]\
    ────また一人、新しいお客様。\\n\
    \\![bind,シルエット,黒塗り2,1]\
    腕に麻の欠片がついているわ。\\n\
    \\![anim,offset,800002,0,-100]\
    夜を狙って、こっそりと忍び込んだのね。\\n\
    \\![anim,offset,800002,0,-150]\
    そうして、私のもとに。\\n\
    \\![anim,offset,800002,0,-250]\
    いつも疑問に思っているのよ。\\n\
    そうまでして得たいものって何かしら？\\n\
    \
    \\![bind,シルエット,黒塗り4,1]\
    h1111201好奇は尊く、そして卑しい。\\n\
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
    h1111210そうね、人間はこんなに赤い目や白い肌をしていない。\\n\
    わかっていても、幽霊にとって人の姿を維持するのはとても難しいことなの。\\n\\n\
    \
    \\1幽霊……まさか本当に……。\\n\\n\
    \
    h1111204目の前にいるのだから、あなたは信じるしかないのよ。\\n\
    h1111310「カンテルベリオの幽霊」、おおかたあの噂を聞いてきたのでしょう。\\n\\n\
    h1112304おめでとう、私が件の幽霊、そしてこの館の主よ。\\n\
    \\x\
    \
    ……h1111210さて、ここを訪れるほどのことだから、あなたは私たちに興味があるのでしょうね。\\n\
    オカルトフリークが廃墟探索、珍しいことじゃないわ。\\n\\n\
    \\1……微妙に違っていたが、少女は有無を言わせぬ口調で続ける。\\n\
    h1111304あなたたちにとって、理外の存在は魅力的だものね？\\n\
    h1111206その自由さ、存在の不思議さ、\\n\
    あるいは、h1111306死そのものへのあこがれ。\\n\
    あなたは少なからず、それらを抱いていた。\\n\
    h1111210だからあなたはここに来たのでしょう。\\n\\n\
    \\1……。\
    h1111210……h1111110……h1111106……。\\n\
    h1121306ふん、あなたのような輩はいつもそう。\
    \\x\
    \
    h1111301…………ねえ、少し頭を使えばわかるはずよ。\\n\
    幽霊がいたとして、\\n\
    自分の住処を悪戯半分で踏み荒らされたとして、\\n\
    それを許すわけがないでしょう？\\n\
    h1111104あなた達はいつも無視している。この、私の気持ちを。\\n\\n\
    \\![bind,シルエット,黒塗り2,1]\\![bind,ex,こわいかお,1]\\f[bold,1]\
    " + &shake("あなたは、考えなかったの？") + "\\f[bold,0]\
    \
    \\1\\![set,balloonwait,0.5]一瞬にして少女の雰囲気が変わった。\\n\
    理解するより先に、身体ががたがた震え出す。\\![set,balloonwait,1]\
    \\x\
    \\1怖い。怖い。\\n\
    頭の中からそれ意外の思考が消える。\\n\
    事情をわかってもらおうなどと、馬鹿げたことだった。\\n\
    これが、こんな存在が現実にいたなんて。\\n\
    空気がどろりと濁って見えるほどの、凄まじい敵意。\\n\
    \
    h1111201\\f[bold,1]……怖い？怖いのでしょうね。\\n\
    もう忘れてしまった感情だわ。至極、どうでもいい。\\n\
    \\n\
    h1111206ふざけた輩には病を憑けて帰しているのだけど、\\n\
    h1111204あなたは……h1111310ふふ、どうしてあげようかしら。\\n\
    糞とはらわたをかき混ぜて殺せば溜飲も下がるかしら？\\n\
    さぞ苦しいでしょうね。\\n\
    その表情を最期の瞬間まで見ていてあげる。\\n\
    \\f[bold,0]\
    \\x\
    \\1……恐怖は依然として消えない。\\n\
    だが、だからこそだろうか。最初の驚きが過ぎていくにつれ、ふと、怒りが込み上げてきた。\\n\
    自分勝手極まりないことだとわかりつつも、頭を覆いつつあるそれは自分でも意外なほど熱く身の内を焦がしていく。\\n\
    \\n\
    ここにはすべてを捨てに来たのだ。なのに、廃墟の主などという、わけのわからない存在にそれを阻まれようとしている。\\n\
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
    h1111110\\![bind,ex,こわいかお,0]\\c…………。h1123211\\![bind,シルエット,,0]くく、ふふふ。\\n\
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
    h1111210なんにせよ、それは止めよ。\\n\
    h1111201その代わりにあなた、しばらくここに居なさい。\\n\\n\
    \
    \\1φ……？\\n\\n\
    \
    h1111204土足で踏み込んだ報いを与えなければね。\\n\
    お咎めなしでは禍根が残るでしょう。\\n\
    h1111206私ではなく、彼らが黙っていないもの。\\n\
    h1111210まあ、今回はある意味では丁度いいわね。\\n\
    \
    \\1……私になにをしろと言うのだろう。\
    \\x\
    \
    h1111206……私はね。ここでずっと、待っているのよ。\\n\
    待つことは嫌いではないけれど、話し相手がいなくて退屈していたところだったの。\\n\
    h1111211だから、お話をしましょう。\\n\\n\
    h1111201私はハイネ。カンテルベリオの幽霊。\\n\
    h1111210訪問者\\f[bold,1]{user_name}\\f[bold,0]、この家の主として、あなたの訪問を歓迎します。\\n\
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
    \\0h1111211協力的で結構なことね。\\n\
    h1111201ところで{user_name}、\\n\
    h1111204幽霊の淹れるお茶を飲んだことはあるかしら？h1111211\
    \\1彼女の微笑みは謎めいて親しげだった。\
    "
});

pub static FIRST_RANDOMTALKS: Lazy<Vec<String>> = Lazy::new(|| {
  vec!["\
      h1111206いま、別の者にお茶の準備をさせているわ。\\n\
      \\n\
      h1111201さて。\\n\
      h1111204それで、ここには死ににきたのですってね。\\n\
      \\n\
      h1113210……相当な覚悟がなければ決断できなかったでしょう。\\n\
      h1113205なにがあなたを追い詰めたのかしらね。\\n\
      仕事、学問、病気、怪我、家庭環境……h1113206あるいは、名前もつけられないほど微妙で、それでいて耐えられないなにか。\
      \\c\
      h1111204ああ、話せなんて言わないわ。\\n\
      話したければそうすればいいけれど、まあ、今はそうではないでしょう？\\n\
      \\n\
      それに、それを聞いて、私があなたを救うこともない。\\n\
      私の立場でできることなどたかが知れているし、\\n\
      あなたの生死は、h1111210なんというか、あまり大きな関心事ではないから。\
      \\x\
      ……h1111201あなたに興味がないわけではないのよ。\\n\
      h1111210もしそうならば引き止めたりせずに追い出しているわ。\\n\
      h1111204勘違いしてほしくないのは、\\n\
      あなたの宿痾に特別な興味はないということ。\\n\
      h1111210あなたの食事の好み、\\n\
      やめられない癖、\\n\
      過去に犯した罪、\\n\
      好みの散歩ルート、\\n\
      なんでもいいの。\\n\
      h1111205この倦み腐った脳に刺激を与えてくれるなら、情報に貴賤はないわ。\
      \\x\
      ……h1111210なにせ、私はここを出られない身だから。\\n\
      h1112205ずっと、ずっと、ずっと、退屈だったの。\\n\
      h1111204そんなところにあなたという玩具が\\n\
      転がりこんできたものだから、\\n\
      都合のいい遊び道具として使おうと考えたのよ。\\n\
      \\n\
      ……h1111210ふふ、ここまで言われても怒らないのね。\\n\
      とても、根深い。h1111211ますます気に入ったわ。\
      \\x\
      h1111204ともかく。逝きたくなったら、そうすればいい。\\n\
      必要ならばいくらか手助けもしましょう。\\n\
      ただ、それまでは私の話し相手になってもらうわ。\\n\
      いいわねφ？\\_w[750]{user_name}。\
      ".to_string(),

      "\
      \\1(コンコン)\\_w[1250]\\n\
      h1111203入りなさい。\\n\\1ハイネの後方、客室のドアが開き、なにかが静かに入ってきた。\\n\
      ……カップとポット。ティーセットだ。浮いている。透明な何かに支えられているかのようだ。\\n\
      \\n\
      h1111203契約している霊よ。召使のようなもの。\\n\
      ……h1141101ああ、h1111204あなたには見えないのかしら？低級霊だものね。\\n\
      \\n\
      ……h1111206そう、ならば伝えておく必要があるわね。\\n\
      この部屋には、常に数人の召使がついているの。\\n\
      h1111201……h1111204驚いた？まあ、普段は居ないのとそう変わらないわ。\\n\
      お茶のおかわりが欲しかったら、h1111306そうね……あのあたりに合図を送ればいいわ。\\n\
      \\x\
      h1111204彼らはみな、私と契約している霊たち。\\n\
      私は力が強いから、それで彼らの……まあ、身の安全を保障しているの。\\n\
      代わりに、彼らは私の命令に従う。\\n\
      これ以上なく忠実にね。\\n\
      \\x\
      h1111203霊にとっての契約は、生者のそれとは重みが違うわ。\\n\
      h1111206なにせ、文字通り存在をかけて結ぶものだから。\\n\
      なまなかな誤魔化しはきかず、反故にすることもできない。\\n\
      h1113210肉体は重くはあれど強固な鎧だったのだと、死んだ後からしみじみ思うわ。\
      \\x\
      h1111204……混乱しているかしら。\\n\
      今すぐにすべてを理解する必要はないわ。\\n\
      どうせ時間はたっぷりあるのだから。\\n\
      \\n\
      ほら、お茶を飲んで。\\n\
      \\1勧められるままに、私は深赤色の液体を口に含んだ。\\n\
      ……まろやかな苦味の後に、薬草を想わせる複雑な香りが広がる。\\n\
      深く、コクのある味わいだ。……美味しい。\\n\
      h1111211悪くないでしょう？h1111204それが私のお気に入り。\\n\
      ……h1111211これで、私の好物を1つ知れたわね。\\n\
      h1111204全て、このように。\\n\
      じっくり、互いを知っていけばいいのよ。\
      ".to_string()
  ]
});

pub static FIRST_CLOSE_TALK: &str = "\
    ならば、送っていきましょう。\\n\
    h1111210安心しなさい、他意などないわ。\\n\
    \\n\
    h1141101逃がして良いのかって？h1121204それ、自分で聞くの？\\n\
    h1111210まあいいわ。h1111204ええ、良いのよ。\\n\
    h1111205こうして縁が結ばれたのだから、あなたはきっとまた来る。\\n\
    h1111208むしろ、自分から戻りたいと思う時が来るでしょう。\\n\
    \\n\
    h1111204さあ、もう行きなさい。\\n\
    h1111210終わりの時が遠いのならば、せめて、良い夢を。\\n\
    また会いましょう、{user_name}。\\n";

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
