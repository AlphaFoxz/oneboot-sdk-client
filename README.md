# 关于本项目

## rust 部分

### pest 模块语法说明

- [pest 文档](https://docs.rs/pest)
- [在线测试](https://pest.rs/#editor)

#### 规则修饰符

- 默认规则

不添加修饰符的时候，token 之间可插入空白字符和注释，生成的规则会包含子规则

1.Silent (`_`)

静默规则，解析后不会创造规则，也不会报告错误

Silent rules do not create token pairs during parsing, nor are they error-reported.

```log
a = _{ "a" }
b =  { a ~ "b" }
```

Parsing `"ab"` produces the token pair `b()`.

2.Atomic (`@`)

原子规则，各个词素之间不自动匹配空白字符串和注释，并且本条规则将成为新的最小规则

Atomic rules do not accept whitespace or comments within their expressions and have a cascading effect on any rule they call. I.e. rules that are not atomic but are called by atomic rules behave atomically.

Any rules called by atomic rules do not generate token pairs.

```log
a =  { "a" }
b = @{ a ~ "b" }

WHITESPACE = _{ " " }
```

Parsing `"ab"` produces the token pair `b()`, while `"a   b"` produces an error.

3.Compound-atomic (`$`)

复合原子规则，各个词素之间不自动匹配空白字符串和注释，但解析后将包含子规则

Compound-atomic are identical to atomic rules with the exception that rules called by them are not forbidden from generating token pairs.

```log
a =  { "a" }
b = ${ a ~ "b" }

WHITESPACE = _{ " " }
```

Parsing `"ab"` produces the token pairs `b(a())`, while `"a   b"` produces an error.

4.Non-atomic (`!`)

非原子规则，与普通规则相同，只是它会阻止原子和复合原子规则的级联效应。

Non-atomic are identical to normal rules with the exception that they stop the cascading effect of atomic and compound-atomic rules.

```log
a =  { "a" }
b = !{ a ~ "b" }
c = @{ b }

WHITESPACE = _{ " " }
```

Parsing both `"ab"` and `"a   b"` produce the token pairs `c(a())`.

#### 特殊规则

- `WHITESPACE` - runs between rules and sub-rules
- `COMMENT` - runs between rules and sub-rules
- `ANY` - matches exactly one `char`
- `SOI` - (start-of-input) matches only when a `Parser` is still at the starting position
- `EOI` - (end-of-input) matches only when a `Parser` has reached its end
- `POP` - pops a string from the stack and matches it
- `POP_ALL` - pops the entire state of the stack and matches it
- `PEEK` - peeks a string from the stack and matches it
- `PEEK[a..b]` - peeks part of the stack and matches it
- `PEEK_ALL` - peeks the entire state of the stack and matches it
- `DROP` - drops the top of the stack (fails to match if the stack is empty)

`WHITESPACE` and `COMMENT` should be defined manually if needed. All other rules cannot be overridden.

`WHITESPACE` 和 `COMMENT`可根据需要，进行重定义。所有的其他特殊规则不可重写

#### 内置规则

- `ASCII_DIGIT` - matches a numeric character from 0..9
- `ASCII_NONZERO_DIGIT` - matches a numeric character from 1..9
- `ASCII_BIN_DIGIT` - matches a numeric character from 0..1
- `ASCII_OCT_DIGIT` - matches a numeric character from 0..7
- `ASCII_HEX_DIGIT` - matches a numeric character from 0..9 or a..f or A..F
- `ASCII_ALPHA_LOWER` - matches a character from a..z
- `ASCII_ALPHA_UPPER` - matches a character from A..Z
- `ASCII_ALPHA` - matches a character from a..z or A..Z
- `ASCII_ALPHANUMERIC` - matches a character from a..z or A..Z or 0..9
- `ASCII` - matches a character from \x00..\x7f
- `NEWLINE` - matches either “\n” or “\r\n” or “\r”

## 前端部分

[Vue3](https://cn.vuejs.org/guide/introduction.html)
[primevue](https://primevue.org/setup/)
