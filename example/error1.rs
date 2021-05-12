use std::io;
use std::fmt;
use std::error::Error;

// 建一個能包裝 io::Error 的 struct
#[derive(Debug)] // 實作 Debug
struct MyError(Option<io::Error>);

impl fmt::Display for MyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // 寫出自訂的錯誤訊息
    f.write_str("這是自訂的錯誤: ")?;
    match self.0 {
      Some(ref err) => {
        // 如果有包裝的 io::Error 就把它的訊息印出來
        write!(f, "{}", err)
      }
      None => {
        write!(f, "沒有包裝的 io::Error")
      }
    }
  }
}

impl Error for MyError {
  // 覆寫原本的 cause ，在如果有原本的 io::Error 時傳回去
  fn cause(&self) -> Option<&Error> {
    // 這邊很可惜沒辦法用 Option::as_ref
    match self.0 {
      Some(ref err) => Some(err),
      None => None,
    }
  }
}

// 從 io::Error 轉換成 MyError
impl From<io::Error> for MyError {
  fn from(err: io::Error) -> Self {
    MyError(Some(err))
  }
}

fn main() {
  let err = MyError(None);
  println!("{}", err);
  let err = MyError(Some(io::Error::new(io::ErrorKind::Other, "Demo")));
  println!("{}", err);
}