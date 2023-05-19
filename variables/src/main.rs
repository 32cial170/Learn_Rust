// 3-1
//const MAX_POINT: u32 = 100_000;
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

//const(常數，常量) 和 immutable 有很多區別
//const 永遠都是不可變的
//const 的類型必須標註
//const 可在任何作用域declare, 包括 gloabal。這讓它們非常有用，讓許多部分的程式碼都能夠知道它們。
//const 最後一個差別是常數只能被常數表達式設置，無法綁定 到函數的return值，無法綁定 任一在運行時才能計算出的數值。
//在程序運行期間，常數在其declare的scope內一直有效
//Rust 的常數命名規則為使用全部英文大寫並用底寫區隔每個單字。

// fn main() {
//     // println!("Hello, world!");
//     let mut x = 5;
//     println!("the value of x is {}", x);
//     x = 6;
//     println!("the value of x is {}", x); --> error: cannot assign twice to immutable variable

// }

// shadowing
// fn main(){
//     let x = 5;
//     let x = x+1;
// }

//shadowing 使用let聲明的同名新變量，它的類型可以與之不同
// fn main(){
//     let space = "    ";
//     let space = space.len();
//     println!("{}",space); --->合法

//     let mut space = "    ";
//     space = space.len();
//     println!("{}",space); --->不合法，型別不match string != u64

// }

// 3-2
//rust是靜態編譯，在編譯時必須知道所有變量類型
//基於使用的值，編譯器通常能推斷其類型
//但有時候如果多種型別都有可能時，像是第二章的「將猜測的數字與祕密數字做比較」用到的 parse 將 String 轉換成數字時，我們就需要像這樣加上型別詮釋：
//let guess: u32 = "42".parse().expect("這不是數字！");
//如果我們沒有像上列程式碼這樣加上型別詮釋 : u32 的話，Rust 將會顯示錯誤訊息。

//u32:unsigned i32:signed
//isize 與 usize 型別則是依據你程式運行的電腦架構來決定大小，如果你在 64 位元架構上的話就是 64 位元；如果你是 32 位元架構的話就是 32 位元。


// 整數溢位

// 假設你有個變數型別是 u8 可以儲存 0 到 255 的數值。如果你想要改變變數的值超出這個範圍的話，比方說像是 256，那麼就會發生整數溢位，
// 這會產生兩種不同的結果。如果你是在除錯模式編譯的話，Rust 會包含整數溢位的檢查，造成你的程式在執行時恐慌（panic）。
// Rust 使用恐慌來表示程式因錯誤而結束。

// 當你是在發佈模式下用 --release 來編譯的話，Rust 則不會加上整數溢位的檢查而造成恐慌。相反地，如果發生整數溢位的話，Rust 會作出二補數包裝的動作。
// 簡單來說，超出最大值的數值可以被包裝成該型別的最低數值。以 u8 為例的話，256 會變成 0、257 會變成 1，以此類推。程式不會恐慌，但是該變數可能會得到一個不是你原本預期的數值。
// 通常依靠整數溢位的行為仍然會被視為邏輯錯誤。

// 要顯式處理可能的溢位的話，你可以使用以下標準函式庫中基本型別提供的一系列方法：
//     將所有操作用 wrapping_* 方法包裝，像是 wrapping_add。
//     使用 checked_* 方法，如果有溢位的話其會回傳 None 數值。
//     使用 overflowing_* 方法，其會回傳數值與一個布林值來顯示是否有溢位發生。
//     屬於 saturating_* ，讓數值溢位時保持在最小或最大值。

// 3-3
// 複合型別可以組合數個數值為一個型別，Rust 有兩個基本複合型別：元組（tuples）和陣列（arrays）(簡中叫"數組")
// 元組（Tuple）型別
// 元組是個將許多個不同型別的值放在一個類型裡
// 元組擁有固定長度，一旦宣告好後，它們就無法增長或縮減。

// 創建 tuple
// 在小括號裡，將值用逗號分開
// 元組的每個位置都對應一個類型，tuple中各元素類型不必相同(不像array裡所有元素類型必需相同)
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// 獲取tuple元素值 
// 我們可以用模式配對（pattern matching）來解構(destructure)一個元組的數值，如以下所示：
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("{}, {}, {}", x, y, z);
// }

// 訪問tuple 內的元素
// 我們可以直接用句號（.）再加上數值的索引來取得元組內的元素。舉例來說：
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     println!("{}, {}, {}", tup.0, tup.1, tup.2);
// }

// 陣列(數組)
// 陣列中的每個型別必須是一樣的。
// 和其他語言的陣列不同，Rust 的陣列是固定長度的。

// 創建 array
// 我們將數值寫在陣列中的括號內，每個數值再用逗號區隔開來：
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// array的用處
// 當你想要你的資料被分配在堆疊（stack）而不是堆積（heap）的話，使用陣列是很好的選擇
// 或者當你想確定你永遠會取得固定長度的元素時也是。
// 所以陣列不像向量（vector）型別那麼有彈性，向量是標準函式庫提供的集合型別，類似於陣列但允許變更長度大小。
// 如果你不確定該用陣列或向量的話，通常你應該用向量就好。
// fn main() {
// let months = ["一月", "二月", "三月", "四月", "五月", "六月", "七月",
//               "八月", "九月", "十月", "十一月", "十二月"];
// }

// array的類型
// 中括號寫出型別和元素個數，並用分號區隔開來，如以下所示：
// let a: [i32; 5] = [1, 2, 3, 4, 5];

// 另一種聲明array的方法
// 如果你想建立的陣列中每個元素數值都一樣的話，你可以指定一個數值後加上分號，最後寫出元素個數。如以下所示：
// let a = [3; 5];
// 這樣寫與 let a = [3, 3, 3, 3, 3]; 的寫法一樣，但比較簡潔。
    
// 訪問array元素
// 一個陣列是被分配在Stack上且已知固定大小的一整塊記憶體，你可以用索引來取得陣列的元素
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }
// 如果訪問的索引超出了 array 的範圍，那麼
// 1. 編譯會通過
// 2. 運行時會報錯(runtime 時會 panic)--> rust不允許其繼續訪問相應地址的內存(相應的記憶體位置)
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[5];-->編譯報錯
//     let index = [12, 13, 14, 15];
//     let second = a[index[0]];--> runtime panic
// }

// 3.4 函式
// 宣告函數使用 fn 關鍵字
// Rust 程式碼使用 snake case 式作為函式與變數名稱的慣例風格 which means 所有的字母都是小寫，並用底線區隔單字。
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("另一支函式。");
// }
// 雖然main宣告在another_function前，但Rust 不在乎你的函式是在哪裡定義的，只需要知道它定義在作用域某處，且能被呼叫者看到就好。所以可以在 main 函式中呼叫another_function。

// 函數的參數
// parameter(定義函式擁有的參數) 和 argument(我們傳遞的數值會叫做argument)

// 在函式宣告中，你必須宣告每個參數的型別像以下例子的(value: i32, unit_label: char)
// fn main() {
//     print_labeled_measurement(5, 'h'); // argument
// }

// fn print_labeled_measurement(value: i32, unit_label: char) { //parameter
//     println!("測量值爲：{value}{unit_label}");
// }

// 函式本體是由一系列的陳述式（statements）(語句)並在最後可以選擇加上表達式（expression)
//  Rust 是門基於表達式（expression-based）的語言
// 陳述式（Statements）是進行一些動作的指令，且不回傳任何數值。
// 表達式（Expressions）則是計算並產生數值。
// 陳述式範例：// 不回傳任何數值
// fn main() {
//     let y = 6;
// }
// 此函式定義(line 178)也是陳述式，整個範例就是本身就是一個陳述式。
// 陳述式不會回傳數值，因此你無法將 let 陳述式賦值給其他變數。如同以下程式碼所做的，你將會得到一個錯誤：
// fn main() {
//     let x = (let y = 6);
// }
// 當你執行此程式時，你就會看到這樣的錯誤訊息：
// expected expression, found statement (`let y = 6`)
// let y = 6 陳述式不回傳數值，所以 x 得不到任何數值。這就和其他語言有所不同，像是 C 或 Ruby，通常它們的賦值仍能回傳所得到的值
// 在那些語言，你可以寫 x = y = 6 同時讓 x 與 y 都取得 6，但在 Rust 就不行。
// 表達式可以是陳述式的一部分
// 在這個例子中 let y = 6; 的 6 其實就是個算出 6 的表達式
// 呼叫函式也可以是表達式、呼叫巨集也是表達式(println!())、我們用 {} 產生的作用域也是表達式。舉例來說：
// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     }; // y = 4
//     // 這個{}就是個表達式，那{}return值就是 x+1
//     // let y = {
//     //     let x = 3;
//     //     x + 1;
//     // }; // 如果在x+1 後面加上分號就變成statement，就沒有return值了
//     // 若最後沒有表達式reuturn，則回傳一個 () 的 tuple (void tuple) 

//     println!("y 的數值為：{y}");
// }

// 函式回傳值
// 我們必須用箭頭（->）來宣告 函式回傳值 的型別，但不可以為回傳值命名
// 在 Rust 中，回傳值其實就是函式本體最後一行的表達式。
// 若想提前返回，可以用 return 關鍵字加上一個數值來提早回傳函式
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("x 的數值為：{x}");
// }
// fn plus_five(x: i32) -> i32 {
//     x + 5
//     // 這裡不能加 ";"
//     // 加了 ";" 會變 statment 並回傳 () tuple

// }

// fn main() {
//     let x = plus_five(6);

//     println!("x 的數值為：{x}");
// }

// 3-5
// 程式碼的條件判斷必須是 bool。如果條件不是 bool 的話，我們就會遇到錯誤 
// fn main() {
//     let number = 3;

//     if number { // error: expeected 'bool', found integer
//         println!("數字為三");
//     }
// }

// 使用太多的 else if 表達式很容易讓你的程式碼變得凌亂，
// 所以當你需要用到一個以上時，那麼最好使用 match 重構程式碼

// 在 let 陳述式中使用 if
// 因為 if 是表達式，所以我們可以把if 放在 let 陳述式的右邊，將結果賦值給變數。
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("數字結果為：{number}");
// }

// 此例中，if 表達式的值取決於哪段程式碼被執行。
// 這代表可能成為最終結果的每一個 if 分支必須要是相同型別。在範例 3-2 中，各分支的型別都是 i32。
// 如果型別不一致的話，如以下範例所示，我們會得到錯誤：
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "六" };

//     println!("數字結果為：{number}");
// }
// 當我們嘗試編譯程式碼時，我們會得到錯誤。if 和 else 分支的型別並不一致
// if 段落的表達式運算出整數，但 else 的區塊卻運算出字串。
// 這樣行不通的原因是變數只能有一個型別。
// Rust 必須在編譯期間確切知道變數 number 的型別，這樣才能驗證它的型別在任何有使用到 number 的地方都是有效的。
// 要是 number 只能在執行時知道的話，Rust 就沒辦法這樣做了。如果編譯器必須追蹤所有變數多種可能存在的型別，那就會變得非常複雜並無法為程式碼提供足夠的保障。

// 使用迴圈重複執行
// Rust 提供三種迴圈：loop、while 和 for。

// loop
// fn main() {
//     loop {
//         println!("再一次！");
//     }
// }
// 你可以在迴圈內加上 break 關鍵字告訴程式何時停止執行迴圈。
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("結果為：{result}");
// }
// 用 break 關鍵字回傳 counter * 2。在迴圈結束後，我們用分號才結束這個賦值給 result 的陳述式。最後我們印出 result，而結果為 20。

// 使用 while 做條件迴圈
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("升空！！！");
// }

// 使用 for 遍歷集合
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("數值為：{element}");
//     }
// }

// Range是標準函式庫提供的型別，用來產生一連串的數字序列，從指定一個數字開始一直到另一個數字之前結束
// rev可以用來反轉這個範圍：
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("升空！！！");
}


// use std::io;

// fn main() {

//     //let x = 2.0;    // 浮點數莫認為f64
//     //let t = true  //bool

//     let a = [1, 2, 3, 4, 5];
    
//     let ind = [12, 13, 14, 15];

//     println!("請輸入陣列索引");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("讀行失敗");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("輸入的索引並非數字");

//     let element = a[index];
//     //let element = a[ind[0]];
//     //let x:i32 = (let y = 2);

//     println!(
//         "索引 {index} 元素的數值爲：{element}"
//     );
// }
