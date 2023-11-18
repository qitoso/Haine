use core::cmp::Ord;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use vibrato::{Dictionary, Tokenizer};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Rank {
  First,
  Second,
  Manual,
  Normal,
}

pub struct Block {
  pub rank: Rank,
  pub text: String,
}

pub struct Inserter {
  cols_num: f32,
  tokenizer: Arc<Mutex<Option<Tokenizer>>>,
  join_handle: Option<JoinHandle<()>>,
}

impl Inserter {
  pub fn new(cols_num: f32) -> Self {
    Inserter {
      cols_num,
      tokenizer: Arc::new(Mutex::new(None)),
      join_handle: None,
    }
  }

  pub fn is_ready(&mut self) -> bool {
    let tokenizer_clone = self.tokenizer.clone();
    let tokenizer = tokenizer_clone.lock().unwrap();
    tokenizer.is_some()
  }

  pub fn start_init(&mut self) {
    self.tokenizer = Arc::new(Mutex::new(None));
    let tokenizer_clone = self.tokenizer.clone();
    self.join_handle = Some(std::thread::spawn(move || {
      let bytes = include_bytes!("../ipadic-mecab-2_7_0/system.dic.zst").to_vec();
      let reader = zstd::Decoder::with_buffer(&bytes[..]).unwrap();
      let dict = Dictionary::read(reader).unwrap();
      *tokenizer_clone.lock().unwrap() = Some(Tokenizer::new(dict));
    }));
  }

  pub fn default() -> Self {
    Self::new(24.0)
  }

  pub fn run(&mut self, src: String) -> String {
    let parts = self.wakachi(src);
    self.render(parts)
  }

  #[allow(dead_code)]
  pub fn tokenize(&mut self, src: String) {
    let tokenizer_clone = self.tokenizer.clone();
    let tokenizer = tokenizer_clone.lock().unwrap();
    let t = tokenizer.as_ref().unwrap();
    let mut worker = t.new_worker();
    worker.reset_sentence(&src);
    worker.tokenize();
    for token in worker.token_iter() {
      println!("{}: {}", token.surface(), token.feature());
    }
  }

  fn wakachi(&mut self, src: String) -> Vec<Block> {
    let tokenizer_clone = self.tokenizer.clone();
    let tokenizer = tokenizer_clone.lock().unwrap();
    let t = tokenizer.as_ref().unwrap();
    let mut worker = t.new_worker();
    let mut text = src.clone();
    let mut _word_counts = vec![0, 0];

    // 候補
    // first: これが含まれていたら改行
    let first = vec![
      "動詞,自立",
      "動詞,接尾",
      "形容詞,自立",
      "形容詞,接尾",
      "形容詞,非自立",
      "副詞,一般",
      "副詞,助詞類接続",
      "接続詞,",
      "助詞,係助詞",
      "助詞,終助詞",
      "助詞,副詞化",
      "記号,句点",
      "記号,読点",
      "記号,一般",
    ];

    // second: これが含まれており、かつ行の後半なら改行
    let second = vec![
      "名詞,副詞可能",
      "助詞,格助詞",
      "助詞,接続助詞",
      "助詞,副助詞",
      "助詞,副助詞／並立助詞／終助詞",
      "助詞,並立助詞",
      "助詞,連体化",
      "記号,括弧閉",
      "フィラー",
    ];

    // forbid: ただし、これが含まれていたら改行しない
    let forbid = ["未然形,"];

    // add_before: これが含まれていたら、前の行に追加
    let add_before = vec![
      "非自立,",
      "接続助詞,",
      "終助詞,",
      "格助詞,",
      "句点,",
      "読点,",
    ];

    // "助詞,格助詞,一般,*,*,*,を"

    let mut results: Vec<Block> = Vec::new();
    let mut result = "".to_string();
    let delim_re = "\x1f$1";
    let delim = "\x1f";
    let line_splitter = Regex::new(r"(\\n|\\_l\[0|\\x|\\c|\\[01]|\\p\[\d+\])").unwrap();
    text = line_splitter.replace_all(&text, delim_re).to_string();
    let lines = text
      .split(delim)
      .map(|s| s.to_string())
      .collect::<Vec<String>>();

    for line in lines {
      let sakura_script_re =
        Regex::new(r###"\\_{0,2}[a-zA-Z0-9*!&](\d|\[("([^"]|\\")+?"|([^\]]|\\\])+?)+?\])?"###)
          .unwrap();

      let mut sakura_scripts = sakura_script_re.find_iter(&line);
      let ss_splitted = sakura_script_re.split(&line).collect::<Vec<&str>>();
      for pieces in ss_splitted {
        worker.reset_sentence(pieces);
        worker.tokenize();
        for token in worker.token_iter() {
          let rank = if forbid.iter().find(|&&p| token.feature().find(p).is_some()) != None {
            Rank::Normal
          } else if first.iter().find(|&&p| token.feature().find(p).is_some()) != None {
            Rank::First
          } else if second.iter().find(|&&p| token.feature().find(p).is_some()) != None {
            Rank::Second
          } else {
            Rank::Normal
          };

          if add_before
            .iter()
            .find(|&&p| token.feature().find(p).is_some())
            != None
            && result.is_empty()
          {
            if results.len() > 0 {
              let last = results.len() - 1;
              // results[last].text += "#";
              results[last].text += token.surface();
            }
          } else {
            result += token.surface();
            if rank != Rank::Normal {
              if result.chars().count() == 1 {
                // TODO: 1文字でいいのか検討
                let last = results.len() - 1;
                results[last].text += &result;
                results[last].rank = rank;
              } else {
                results.push(Block { rank, text: result });
              }
              result = "".to_string();
            }
          }
        }
        if let Some(s) = sakura_scripts.next() {
          result += s.as_str();
        }
      }
      results.push(Block {
        rank: Rank::Manual,
        text: result,
      });
      result = "".to_string();
    }

    for r in results.iter() {
      println!("{:?}: {}", r.rank, r.text);
    }

    results
  }

  fn render(&mut self, parts: Vec<Block>) -> String {
    let re_open_bracket = Regex::new(r"[「『（【]").unwrap();
    let re_close_bracket = Regex::new(r"[」』）】]").unwrap();
    let re_periods = Regex::new(r"[、。！？]").unwrap();
    let re_change_scope = Regex::new(r"(\\[01][^w]?|\\p\[\d+\])").unwrap();
    let re_not_number = Regex::new(r"[^\d]").unwrap();
    let re_change_line = Regex::new(r"(\\n|\\_l\[0[,0-9em%]+\]|\\x|\\c)").unwrap();
    let mut result = String::new();
    let mut counts = vec![0.0, 0.0];
    let mut i = 0;
    let mut scope: usize = 0;
    let mut brackets_depth: i32 = 0;
    loop {
      if i >= parts.len() {
        break;
      }
      let rank = &parts[i].rank;
      let part = parts[i].text.clone();
      let c = self.count(part.to_string());
      brackets_depth += re_open_bracket.find_iter(&part).count() as i32;
      brackets_depth -= (re_close_bracket.find_iter(&part).count() as i32).max(0);

      if re_change_scope.is_match(&part) {
        let c = re_change_scope.captures(&part).unwrap()[0].to_string();
        scope = re_not_number.replace_all(&c, "").parse::<usize>().unwrap();
      }

      if re_change_line.is_match(&part) {
        counts[scope] = 0.0;
      }

      if c > self.cols_num {
        result.push_str(&part);
        counts[scope] += c % self.cols_num;
        counts[scope] %= self.cols_num;
        i += 1;
        continue;
      }
      let after_counts = counts[scope] + c;
      if after_counts > self.cols_num && *rank == Rank::First {
        // result.push_str(&format!("f{}\\n", counts[scope]));
        result.push_str("\\n");
        counts[scope] = 0.0;
        continue;
      } else if after_counts > self.cols_num && *rank == Rank::Second {
        // result.push_str(&format!("s{}\\n", counts[scope]));
        result.push_str("\\n");
        counts[scope] = 0.0;
        continue;
      }
      counts[scope] += c;
      result.push_str(&part);

      // 句読点後の文章が1行に収まるなら、一気に出力して次へ
      // ただし括弧内では実行しない ぶつ切りの引用文とか台詞はなんか変な感じがするので
      if re_periods.is_match(&part) && brackets_depth == 0 {
        let mut j = i + 1;
        let mut next_line = String::new();
        while j < parts.len() {
          let next = parts[j].text.clone();
          if re_change_scope.is_match(&next) || re_change_line.is_match(&next) {
            j -= 1;
            break;
          }
          next_line.push_str(&next);
          j += 1;
        }

        let next_word_count = self.count(next_line.clone());
        if next_word_count <= self.cols_num {
          // 句読点の後に改行を入れるのは、句読点の後の文章が1行に収まらない場合のみ
          if counts[scope] + next_word_count > self.cols_num {
            // result.push_str("@\\n");
            result.push_str("\\n");
            counts[scope] = 0.0;
          } else {
            // result.push_str(";");
          }
          result.push_str(&next_line);
          counts[scope] += next_word_count;
          i = j;
        }
      }
      i += 1;
    }
    result
  }

  fn count(&self, text: String) -> f32 {
    let sakura_script_re =
      Regex::new(r###"\\_{0,2}[a-zA-Z0-9*!&](\d|\[("([^"]|\\")+?"|([^\]]|\\\])+?)+?\])?"###)
        .unwrap();
    let removed = sakura_script_re.replace_all(&text, "");
    let mut count = 0.0;
    count += removed.chars().filter(|c| c.is_ascii()).count() as f32 * 0.5;
    count += removed.chars().filter(|c| !c.is_ascii()).count() as f32;
    count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::events::aitalk::TALKS;

  #[test]
  fn inserter() {
    for t in TALKS.iter() {
      let text = t.to_string();
      let mut ins = Inserter::default();
      ins.start_init();
      while !ins.is_ready() {
        std::thread::sleep(std::time::Duration::from_millis(100));
      }
      ins.tokenize(text.clone());
      println!("\n{}\n", ins.run(text).replace("\\n", "\n"));
    }
  }
}
