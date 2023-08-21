
# 介绍

使用Rust，实现了函数图像绘制语言解释器。其语法分析基于递归下降。

- 表达式支持四则运算、乘方、函数、括号。
- 支持行注释。
- 使用FOR语句来绘制点以画出函数图像。
- 支持平移（ORIGIN）、放大（SCALE）、旋转（ROT）。这三个操作只会影响后面绘制的点。
- 支持**定义变量**，可以**动态置换变量**。
- 可以对不同文件分别解释，然后以不同的颜色画在同一张图上。
- 高质量的异常体系（目前只有Error没有Warning），支持细节打印、定位、期望提示，具体见下文。
- 递归下降语法分析器，简洁而高效的词法分析器，低内存消耗、带缓存的文本文件读取器。

>表达式支持的函数的语法分析接口实际上支持任意数量的参数。因此可以随意定义多参数的变量，不过这样的话可能每个函数调用都要返回Result以表明参数数量正确性了。
>同时，这也留下了自定义多参函数的可能性，如 `fn my_function(arg_name1,arg_name_2) => 3+T\*arg_name1+arg_name2;`，尚待实现。

# 文法

CFG上下文无关文法如下所示，但写得并不完美，使用了一些正则表达式符号（用markdown的`代码段`标记框柱的都是正则表达式或文法符号，而非字面量）。因为使用的是LL(1)文法进行最左推导，如果要避免左递归的话加减乘除都只能是右结合了。因此，递归到加减乘除的时候变成迭代处理，以实现左结合。

1. PROGRAM `->` STATEMENT ;
2. STATEMENT `->` ORIGIN_STATEMENT 
`|` SCALE_STATEMENT 
`|` ROT_STATEMENT 
`|` FOR_STATEMENT
`|` DEF_STATEMENT
`|` LET_STATEMENT
3. ORIGIN_STATEMENT `->` origin is ( EXPRESSION , EXPRESSION )
4. SCALE_STATEMENT `->` scale is ( EXPRESSION , EXPRESSION )
5. ROT_STATEMENT `->` rot is EXPRESSION
6. FOR_STATEMENT `->` for $variable from EXPRESSION to EXPRESSION step EXPRESSION draw ( EXPRESSION , EXPRESSION )
7. DEF_STATEMENT `->` def $variable = EXPRESSION
8. LET_STATEMENT `->` let $variable = EXPRESSION
9. EXPRESSION `->` TERM `(` `(` + `|` - `)` TERM `)*`  **//加减**
10. TERM `->` + FACTOR `(` `(` \* `|` / `)` FACTOR `)*`  **//乘除**
11. FACTOR `->` `(` + `|` - `)?` COMPONENT  **//正负号**
12. COMPONENT `->` ATOM `(` \*\* COMPONENT `)?` **//乘方**
13. ATOM `->` $id **//数字字面量**
`|` ( EXPRESSION )  **//括号**
`|` $variable  **//变量**
`|` $funcion ( EXPRESSION , EXPRESSION ) **//函数**



乘方是右结合的，加减乘除是左结合的。

# 语法树

由于是解释器，实际上需要复用的只有表达式EXPRESSION，因此此处语法树都是针对EXPRESSION的。

封装了比较好的语法树打印功能。下面给出一些语法树的例子：

```rust
ROT is 8*(2--5+3);

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
ROT is 1+2**3**4/((5+6)/7);

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

涉及变量的语法树见下文。
# 变量

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

注意看倒数第二课树以及输出，很显然，hololive的重定义并不会影响Haachama：

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

## 动态置换 Let

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

分析中所有的异常都会以dyn Exception的形式向上传递，汇集到ParserManager的parse处进行打印。

>非保留字的、字母带头、只包含字母和数字的词都会被认为是变量，因此写错词可能也会被识别为变量。
## 静态异常

涉及词法分析和语法分析的异常。

### 非法词

一般是数字开头又带字母的词。

```rust
ROT iis 2.1+42*2/4;
```

```rust
Interpret Terminated at 1:8

*** Analysis Error ***
Syntax Error: Token { token_type: Variable, lexeme: "IIS" }
Expect: [Is]
Found : Variable
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
## 运行时异常

与其说是运行时异常，不如描述成“语义级异常”。

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
Interpret Terminated at 3:0

*** Runtime Error ***
Undefined Variable Error: "VAR2"
```
# TODO

- 块注释
- 进一步适配多参数函数
- 自定义函数
- 图例





