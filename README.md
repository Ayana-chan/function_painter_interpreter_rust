
# 基本介绍

使用Rust，实现了函数图像绘制语言解释器。

- 表达式支持四则运算、乘方、函数、括号。
- 函数支持多参与可变参。
- 支持行注释。
- 使用FOR语句来绘制点以画出函数图像。
- 支持平移（ORIGIN）、放大（SCALE）、旋转（ROT）。这三个操作只会影响后面绘制的点。
- 支持**定义表达式变量**、**置换表达式变量**。
- 可以对不同文件分别解释，然后以不同的颜色画在同一张图上。
- 高质量的异常体系（目前只有Error没有Warning），支持细节打印、定位、期望提示，具体见下文。
- 递归下降语法分析器，简洁而高效的词法分析器，低内存消耗、带缓存的文本文件读取器。
- 绘图时支持自动计算坐标轴范围以显示所有已绘制的点。

>表达式支持的函数的语法分析接口通用地支持任意数量的参数。因此可以随意定义多参、变参的函数。
>同时，这也留下了自定义多参、变参函数的可能性，如 `fn my_function(arg_name1,arg_name_2) => 3+T\*arg_name1+arg_name2;`，尚待实现。

# 简单使用

所有字母不分大小写。

变量名必须为字母带头、只能包含字母或数字的连续串。

所有度数采用弧度制。所有旋转均为逆时针。

For后的参数必须为T。T也是程序一开始就可以直接使用的变量，且禁止定义或置换。

如果将caption设为空串，则图中不会显示标题。

请留意不要出现负数的小数次方，会被丢弃。

>支持的符号都定义在`interpreter/src/lexer/token_manager.rs`中。

## 基础实例

输入1：

```rust
//test_file1.txt
For T from 1 to 100 step 0.2 draw(T/10,3*ln(T)); //绘制对数函数
ROT is PI/4; //让之后的图像都逆时针旋转45度
Def quadratic = T**2-5*T-3; //定义一个二次函数的表达式变量
For T from -10 to 18 step 0.2 draw( T/5 , quadratic/10 ); //绘制二次函数
```

输入2：

```rust
Scale is (0.5,3);  
For T from -50 to 100 step 0.2 draw(T,sin(T));
```

main函数：

```rust
fn main() -> Result<(), Box<dyn std::error::Error>>{  
    //指定输入1  
    let aim_file1 = File::open("test_file1.txt").unwrap();  
    let mut interpreter_obj1 = interpreter::Interpreter::new(aim_file1);  
    //限制坐标范围  
    interpreter_obj1.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);  
    let point_result1 = interpreter_obj1.interpret().unwrap();  
  
    //指定输入2  
    let aim_file2 = File::open("test_file2.txt").unwrap();  
    let mut interpreter_obj2 = interpreter::Interpreter::new(aim_file2);  
    //限制坐标范围  
    interpreter_obj2.set_coordinate_range(-10.0, 20.0, -10.0, 20.0);  
    let point_result2 = interpreter_obj2.interpret().unwrap();  
  
    let mut drawer_obj = drawer::Drawer::new()  
        //指定输出图像大小  
        .build_image_size(1280, 720)  
        //指定坐标轴显示范围  
        .build_coordinate_range(-10.0, 20.0, -10.0, 20.0)  
        //指定输出文件名和标题  
        .build_message("draw_test.png", "First Test");  
  
    //添加点集和颜色  
    drawer_obj.add_task(point_result1, drawer::colors::RED);  
    drawer_obj.add_task(point_result2, drawer::colors::BLUE);  
    drawer_obj.draw()  
}
```

结果图像`draw_test.png`：

![400](README_source/draw_test1.png)

>图像其实被拉长了，因为虽然横坐标轴和纵坐标轴的范围是一样的，但是长度不同。

## 函数

>可以查看`interpreter/src/lexer/token_manager.rs`中的`pub fn generate_token_match_map() -> HashMap<String, Token>`函数来浏览所支持的函数及其逻辑。

- 单参: sin,cos,tan,ln,exp,sqrt,abs
- 双参: max,min
- 变参: aver

下面是`For T from -10 to 10 step 0.2 draw(T*2,aver(3*sin(T*2),T**2,-exp(T)));`画出的结果：

![400](README_source/draw_test2.png)

## 自动计算坐标轴范围

如果在建立Drawer时不使用`.build_coordinate_range(-10.0, 20.0, -10.0, 20.0)`来指定坐标轴范围的话，就会自动计算坐标轴范围，可以刚好容纳所有绘出的点。输出图像如下图所示：

![400](README_source/draw_test3.png)

# 文法

EBNF表示如下所示（用markdown的`代码段`标记框柱的都是正则表达式或文法符号，而非字面量）。这里不好写成纯CFG，因为使用的是LL(1)文法进行最左推导，如果要避免左递归的话加减乘除都只能是右结合了。因此，递归到加减乘除的时候变成迭代处理，以实现左结合。

1. PROGRAM = STATEMENT ;
2. STATEMENT = ORIGIN_STATEMENT 
`|` SCALE_STATEMENT 
`|` ROT_STATEMENT 
`|` FOR_STATEMENT
`|` DEF_STATEMENT
`|` LET_STATEMENT
3. ORIGIN_STATEMENT = origin is ( EXPRESSION , EXPRESSION )
4. SCALE_STATEMENT = scale is ( EXPRESSION , EXPRESSION )
5. ROT_STATEMENT = rot is EXPRESSION
6. FOR_STATEMENT = for $variable from EXPRESSION to EXPRESSION step EXPRESSION draw ( EXPRESSION , EXPRESSION )
7. DEF_STATEMENT = def $variable = EXPRESSION
8. LET_STATEMENT = let $variable = EXPRESSION
9. EXPRESSION = TERM `{` `(` + `|` - `)` TERM `}`  **//加减**
10. TERM = FACTOR `{` `(` \* `|` / `)` FACTOR `}`  **//乘除**
11. FACTOR = `[` + `|` - `]` COMPONENT  **//正负号**
12. COMPONENT = ATOM `[` \*\* COMPONENT `]` **//乘方**
13. ATOM = $id **//数字字面量**
`|` ( EXPRESSION )  **//括号**
`|` $variable  **//变量**
`|` $funcion ( EXPRESSION , EXPRESSION ) **//函数**



乘方是右结合的，加减乘除是左结合的。

# 语法树

由于是解释器，实际上需要复用的只有表达式EXPRESSION，因此此处语法树都是针对EXPRESSION的。

封装了比较好的语法树打印功能。下面给出一些语法树的例子：

```rust
8*(2--5+3);

->/$ Mul
  |
  |----->/$ 8.0
  |       `
  |----->/$ Plus
  |       |
  |       |----->/$ Minus
  |       |       |
  |       |       |----->/$ 2.0
  |       |       |       `
  |       |       |----->/$ Minus
  |       |       |       |
  |       |       |       |----->/$ 0.0
  |       |       |       |       `
  |       |       |       |----->/$ 5.0
  |       |       |       |       `
  |       |       |       `
  |       |       `
  |       |----->/$ 3.0
  |       |       `
  |       `
  `
```

```rust
1+2**3**4/((5+6)/7);

->/$ Plus
  |
  |----->/$ 1.0
  |       `
  |----->/$ Div
  |       |
  |       |----->/$ Power
  |       |       |
  |       |       |----->/$ 2.0
  |       |       |       `
  |       |       |----->/$ Power
  |       |       |       |
  |       |       |       |----->/$ 3.0
  |       |       |       |       `
  |       |       |       |----->/$ 4.0
  |       |       |       |       `
  |       |       |       `
  |       |       `
  |       |----->/$ Div
  |       |       |
  |       |       |----->/$ Plus
  |       |       |       |
  |       |       |       |----->/$ 5.0
  |       |       |       |       `
  |       |       |       |----->/$ 6.0
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ 7.0
  |       |       |       `
  |       |       `
  |       `
  `
```

```rust
2**aver(3*T,T**2,exp(T))+1;

->/$ Plus
  |
  |----->/$ Power
  |       |
  |       |----->/$ 2.0
  |       |       `
  |       |----->/$ Func
  |       |       |: AVER
  |       |       |
  |       |       |----->/$ Mul
  |       |       |       |
  |       |       |       |----->/$ 3.0
  |       |       |       |       `
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ Power
  |       |       |       |
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       |----->/$ 2.0
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ Func
  |       |       |       |: EXP
  |       |       |       |
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       `
  |       |       `
  |       `
  |----->/$ 1.0
  |       `
  `
```

涉及变量的语法树见下文。
# 表达式变量

## 定义 Def

可以使用Def定义一个表达式变量。对同一个变量多次Def的效果为普通的覆盖，**不影响之前的相关变量**。

```rust
//Test: Independence and Coverage
Def hololive = (114514+1)*2 + T;  
For T from 1 to 3 step 1 draw(T , 1 + hololive/2 + T);  
  
Def Haachama = hololive + 1919810.5;  
Def hololive = T; //reset variable  
For T from 1 to 3 step 1 draw(Haachama , 1 + hololive/2 + T);
```

上面的输入定义了hololive变量，然后将其作为参数赋值给了Haachama，然后又重新定义了hololive。控制台输出如下：

```rust
Debug: parse a statement, begin token: Token { token_type: Def, lexeme: "DEF", }
->/$ Plus
  |
  |----->/$ Mul
  |       |
  |       |----->/$ Plus
  |       |       |
  |       |       |----->/$ 114514.0
  |       |       |       `
  |       |       |----->/$ 1.0
  |       |       |       `
  |       |       `
  |       |----->/$ 2.0
  |       |       `
  |       `
  |----->/$ T
  |       `
  `
Debug: parse a statement, begin token: Token { token_type: For, lexeme: "FOR", }
->/$ 1.0
  `
->/$ 3.0
  `
->/$ 1.0
  `
->/$ T
  `
->/$ Plus
  |
  |----->/$ Plus
  |       |
  |       |----->/$ 1.0
  |       |       `
  |       |----->/$ Div
  |       |       |
  |       |       |----->/$ Variable
  |       |       |       |: HOLOLIVE
  |       |       |       |----->/$ Plus
  |       |       |       |       |
  |       |       |       |       |----->/$ Mul
  |       |       |       |       |       |
  |       |       |       |       |       |----->/$ Plus
  |       |       |       |       |       |       |
  |       |       |       |       |       |       |----->/$ 114514.0
  |       |       |       |       |       |       |       `
  |       |       |       |       |       |       |----->/$ 1.0
  |       |       |       |       |       |       |       `
  |       |       |       |       |       |       `
  |       |       |       |       |       |----->/$ 2.0
  |       |       |       |       |       |       `
  |       |       |       |       |       `
  |       |       |       |       |----->/$ T
  |       |       |       |       |       `
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ 2.0
  |       |       |       `
  |       |       `
  |       `
  |----->/$ T
  |       `
  `
Debug: Add Point: (1.0, 114517.5)
Debug: Add Point: (2.0, 114519.0)
Debug: Add Point: (3.0, 114520.5)
Debug: parse a statement, begin token: Token { token_type: Def, lexeme: "DEF", }
->/$ Plus
  |
  |----->/$ Variable
  |       |: HOLOLIVE
  |       |----->/$ Plus
  |       |       |
  |       |       |----->/$ Mul
  |       |       |       |
  |       |       |       |----->/$ Plus
  |       |       |       |       |
  |       |       |       |       |----->/$ 114514.0
  |       |       |       |       |       `
  |       |       |       |       |----->/$ 1.0
  |       |       |       |       |       `
  |       |       |       |       `
  |       |       |       |----->/$ 2.0
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ T
  |       |       |       `
  |       |       `
  |       `
  |----->/$ 1919810.5
  |       `
  `
Debug: parse a statement, begin token: Token { token_type: Def, lexeme: "DEF", }
->/$ T
  `
Debug: parse a statement, begin token: Token { token_type: For, lexeme: "FOR", }
->/$ 1.0
  `
->/$ 3.0
  `
->/$ 1.0
  `
->/$ Variable
  |: HAACHAMA
  |----->/$ Plus
  |       |
  |       |----->/$ Variable
  |       |       |: HOLOLIVE
  |       |       |----->/$ Plus
  |       |       |       |
  |       |       |       |----->/$ Mul
  |       |       |       |       |
  |       |       |       |       |----->/$ Plus
  |       |       |       |       |       |
  |       |       |       |       |       |----->/$ 114514.0
  |       |       |       |       |       |       `
  |       |       |       |       |       |----->/$ 1.0
  |       |       |       |       |       |       `
  |       |       |       |       |       `
  |       |       |       |       |----->/$ 2.0
  |       |       |       |       |       `
  |       |       |       |       `
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       `
  |       |       `
  |       |----->/$ 1919810.5
  |       |       `
  |       `
  `
->/$ Plus
  |
  |----->/$ Plus
  |       |
  |       |----->/$ 1.0
  |       |       `
  |       |----->/$ Div
  |       |       |
  |       |       |----->/$ Variable
  |       |       |       |: HOLOLIVE
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       `
  |       |       |----->/$ 2.0
  |       |       |       `
  |       |       `
  |       `
  |----->/$ T
  |       `
  `
Debug: Add Point: (2148841.5, 2.5)
Debug: Add Point: (2148842.5, 4.0)
Debug: Add Point: (2148843.5, 5.5)

```

注意看倒数第二棵树以及输出，很显然，hololive的重定义并不会影响Haachama：

```rust
->/$ Variable
  |: HAACHAMA
  |----->/$ Plus
  |       |
  |       |----->/$ Variable
  |       |       |: HOLOLIVE
  |       |       |----->/$ Plus
  |       |       |       |
  |       |       |       |----->/$ Mul
  |       |       |       |       |
  |       |       |       |       |----->/$ Plus
  |       |       |       |       |       |
  |       |       |       |       |       |----->/$ 114514.0
  |       |       |       |       |       |       `
  |       |       |       |       |       |----->/$ 1.0
  |       |       |       |       |       |       `
  |       |       |       |       |       `
  |       |       |       |       |----->/$ 2.0
  |       |       |       |       |       `
  |       |       |       |       `
  |       |       |       |----->/$ T
  |       |       |       |       `
  |       |       |       `
  |       |       `
  |       |----->/$ 1919810.5
  |       |       `
  |       `
  `
Debug: Add Point: (2148841.5, 2.5)
Debug: Add Point: (2148842.5, 4.0)
Debug: Add Point: (2148843.5, 5.5)
```

## 置换 Let

如果将hololive的重定义语句的Def换成Let，就可以动态地置换变量，也就是说**会影响该变量在Let之前、最近一次的Def之后相关的所有其他变量**。

注意，let必须要求变量已被定义（Def），否则会报“未定义变量”错误。

```rust
Def hololive = (114514+1)*2 + T;  
For T from 1 to 3 step 1 draw(T , 1 + hololive/2 + T);  
  
Def Haachama = hololive + 1919810.5;  
Let hololive = T; //reset variable  
For T from 1 to 3 step 1 draw(Haachama , 1 + hololive/2 + T);
```

上述代码将hololive动态置换为`T`后，打印的Haachama树与最终输出如下，可见Haachama也受到影响了：

```rust
->/$ Variable
  |: HAACHAMA
  |----->/$ Plus
  |       |
  |       |----->/$ Variable
  |       |       |: HOLOLIVE
  |       |       |----->/$ T
  |       |       |       `
  |       |       `
  |       |----->/$ 1919810.5
  |       |       `
  |       `
  `

Debug: Add Point: (1919811.5, 2.5)
Debug: Add Point: (1919812.5, 4.0)
Debug: Add Point: (1919813.5, 5.5)
```

# 异常

分析中所有的异常都会以dyn Exception的形式向上传递，汇集到ParserManager的parse处调用打印。

>非保留字的、字母带头、只包含字母和数字的词都会被认为是变量，因此写错词可能也会被识别为变量。

各个异常都会打印报错位置，并且都具有详尽的信息。

## 分析级异常

涉及词法分析和语法分析的异常。

### 非法词

一般是数字开头又带字母的词。

```rust
Let 1var = 12/T;
```

```rust
Interpret Terminated at 1:9

*** Analysis Error ***
Illegal Symbol: 1VAR
```

### 语法错误

**这个错误会提示“应当是什么词”（Expect）。** 而且如果有多种expect会一起打印出来。

```rust
ROT is 2.1+42*2/4;  
45 is 8;
```

```rust
Interpret Terminated at 2:3

*** Analysis Error ***
Syntax Error: Token { token_type: ConstId, lexeme: "45" }
Expect: [Origin, Scale, Rot, For, Def, Let]
Found : ConstId
```

表达式里面也会进行提示：

```rust
ROT is 8*2---5; //注意，两个连减号就没问题了，等价于-(-5)
```

```rust
Interpret Terminated at 1:14

*** Analysis Error ***
Syntax Error: Token { token_type: Minus, lexeme: "-" }
Expect: [ConstId, Variable, LBracket, Func]
Found : Minus
```

理所当然也能涵盖括号匹配：

```rust
ROT is 8*(2**(5-2);
```

```rust
Interpret Terminated at 2:0

*** Analysis Error ***
Syntax Error: Token { token_type: Semico, lexeme: ";" }
Expect: [RBracket]
Found : Semico
```

## 语义级异常

起名为Runtime Error。

修正：所有包含除以0或无穷大的点都被直接丢掉而非报错。

>由于表达式往往扫完一整句后才会进行计算，所以某些语义级异常的报错位置（解释停止位置）可能有些奇怪。

### 未定义变量

表达式中发现未定义的变量就会报错：

```rust
ROT is 8*var1+5;
```

```rust
Interpret Terminated at 1:14

*** Runtime Error ***
Undefined Variable Error: "VAR1"
```

Let的时候目标变量尚未被定义也会报错：

```rust
Def var1 = 4*T;  
Let var2 = 12/T;
```

```rust
Interpret Terminated at 2:11

*** Runtime Error ***
Undefined Variable Error: "VAR2"
```

### 参数数量不匹配

参数多了：

```rust
For T from 1 to 100 step 0.2 draw(ln(2*T,8),3);
```

```rust
Interpret Terminated at 2:0

*** Runtime Error ***
Arguments' Number not Match Error:
At Function : "LN"
Expect : 1
Receive: 2
```

参数少了：

```rust
For T from 1 to 100 step 0.2 draw(max(2*T),3);
```

```rust
Interpret Terminated at 2:0

*** Runtime Error ***
Arguments' Number not Match Error:
At Function : "MAX"
Expect : 2
Receive: 1
```

可变参数函数的参数少了：

```rust
For T from -10 to 10 step 0.2 draw(T*2,aver());
```

```rust
Interpret Terminated at 2:0

*** Runtime Error ***
Arguments' Number not Match Error:
At Function : "AVER"
Expect : 1+
Receive: 0
```


# TODO

- 块注释
- 自定义函数
- 图例
- 坐标范围、颜色等东西都是可以直接嵌入到语言当中的，不然main函数太长了。
- 让输出图像的纵横比也可以动态变化，可以在用户的要求下防止纵横坐标范围不同导致的畸变。






