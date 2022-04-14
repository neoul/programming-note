# Go handbook

- [Go handbook](#go-handbook)
  - [commands](#commands)
  - [environment](#environment)
  - [Build, install and execute a Go executable binary](#build-install-and-execute-a-go-executable-binary)
  - [Building Go Executables for Multiple Platforms](#building-go-executables-for-multiple-platforms)
  - [Package](#package)
    - [import other packages](#import-other-packages)
  - [Go Syntax](#go-syntax)
    - [type syntax](#type-syntax)
    - [variables](#variables)
    - [constant](#constant)
    - [Comment](#comment)
    - [control](#control)
    - [type conversion](#type-conversion)
    - [array, slice, map](#array-slice-map)
    - [make() and new()](#make-and-new)
    - [function](#function)
    - [Pointers](#pointers)
    - [Structure (field and method)](#structure-field-and-method)
    - [Anonymous fields in Go Structure](#anonymous-fields-in-go-structure)
    - [Interface (collections of methods)](#interface-collections-of-methods)
    - [empty interface](#empty-interface)
    - [Errors](#errors)
    - [Concurrency (goroutine)](#concurrency-goroutine)
    - [Channel](#channel)
    - [Channel Buffering](#channel-buffering)
    - [Channel Synchronization (Waitgroup)](#channel-synchronization-waitgroup)
    - [Select (for Channel)](#select-for-channel)
    - [context](#context)
    - [cgo](#cgo)
    - [Command-Line](#command-line)
    - [empty structure](#empty-structure)
    - [structTag](#structtag)
    - [reflection](#reflection)
    - [Type assertions](#type-assertions)
    - [Type switches](#type-switches)
  - [cgo (golang with C)](#cgo-golang-with-c)
  - [go with gRPC](#go-with-grpc)
  - [Useful commands](#useful-commands)
  - [Useful functions](#useful-functions)
    - [panic() and recover()](#panic-and-recover)
    - [os.Exit()](#osexit)
  - [Useful documents](#useful-documents)
  - [gRPC with simple password authentication](#grpc-with-simple-password-authentication)
  - [vscode with golang](#vscode-with-golang)
  - [Go Fuzzing](#go-fuzzing)

## commands

Commands of golang to use

- `go env`: show environment variables for golang.
- `go build`: `<SOURCE>`: Compile `<SOURCE>` and place it to `$PWD`
- `go install`: `<SOURCE>`: Compile and Install `<SOURCE>` to `$GOBIN`.

## environment

- `$GOROOT`: The path of Go binary distribution (default: `/usr/local/go`)
- `$GOPATH`: The path to Go packages to install and build
- `$GOBIN`: The path to Go executable binaries built

```bash
#!/bin/bash
export GO111MODULE=off
export GOPATH=$HOME/go
export GOBIN=$GOPATH/bin
```

## Build, install and execute a Go executable binary

```bash
cd go-project
source env.sh
go install src/hello/hello.go
hello
```

## Building Go Executables for Multiple Platforms

```bash
CFLAGS="-I$INCLUDE" CPPFLAGS="-I$INCLUDE" CC=$TARGET_ARCHITECTURE-gcc GOOS=linux GOARCH=arm64 CGO_ENABLED=1 go build
```

## Package

Every Go program is made up of packages. Programs start running in package main.

### import other packages

```go
import {
    "fmt"
    "math/rand"
}
```

## Go Syntax

### type syntax

- 타입 후위 정의: 변수 선언시 type은 뒤에 명시
- Go는 정적 타입 프로그래밍 언어

```go
// integer types
uint8, uint16, uint32, uint64, int8, int16, int32, int64

// floating point number
// 0/0 = NaN으로 표현
float32, float64

// complex number
complex64, complex128

x int
p *int // pointer
a [3]int // array

// function declaration
func main(argc int, argv []string) int

// function variable
f func(func(int,int) int, int) int

// function variable (return a function)
f func(func(int,int) int, int) func(int, int) int

// 선언과 함께 함수 구현/ 할당
sum := func(a, b int) int { return a+b } (3, 4)
```

### variables

- `var` keyword 사용
- `:=` 축약형 존재 (`var` 생략)
- namespace: global vs local

```go
var x string
x = "hello world"
var y float32 = 10.0
// implicit variable declaration and value assignment
z := "goodbye world"
var (
    a = 1
    b = 2
    c = 10
)
```

### constant

- `const` keyword 사용
- code 내에서 값 변경 불가

```go
const x string = "Hello World"
x = "Some other string" // error
// e.g.
math.Pi // const variable
```

### Comment

```go
// line comment
/* block comment */
```

### control

```go
func control() {
    // loop statement
    i := 1
    for i <= 10 {
        i = i + 1
    }
    for i := 1; i <= 10; i++ {
    }
    // array loop using range keyword
    x := [5]float64{ 98, 93, 77, 82, 83 }
    for i, value := range x {
    }
    // array loop without index
    for _, value := range x {
    }

    // if else statement
    if i % 2 == 0 {
        // even
    }
    else {
        // odd
    }
    // if statement with initialization
    if num := 9; num < 0 {
    } else if num < 10 {
    } else {
    }

    // if statement with map (dict or hash)
    elements := make(map[string]string)
    elements["H"] = "Hydrogen"
    if name, ok := elements["Un"]; ok {
        fmt.Println(name, ok)
    }

    // switch statement
    switch i {
        case 0: fmt.Println("zero")
        case 1: fmt.Println("one")
        case 2: fmt.Println("two")
        case 3: fmt.Println("three")
        default: fmt.Println("unknown")
    }
}
```

### type conversion

```go
var x int = 10
float64(x)
```

### array, slice, map

```go
// array
var x [5]int
x[0] = 10
x[1] = 20
x[2] = 30
x[3] = 40
x[4] = 50
// [10 20 30 40 50]
x := [5]float64{ 98, 93, 77, 82, 83 }

// slice (max length이상 증가 x)
var x []float64
x := make([]float64, 5) // create a slice with 5 length using make()
x := make([]float64, 5, 10) // create a slice with 5 length. 10 is the length of base array.

// arr: array, x: slice for arr slicing
arr := []float64{1,2,3,4,5}
x := arr[0:5]
arr[:] == arr[0:len(arr)] // true
arr[:5] == arr[0:5] // true

// slice append
slice1 := []int{1,2,3} // [1,2,3]
slice2 := append(slice1, 4, 5) // [1,2,3,4,5]
fmt.Println(slice1, slice2)

// slice copy
slice1 := []int{1,2,3}
slice2 := make([]int, 2)
copy(slice2, slice1) // [1,2,3]
fmt.Println(slice1, slice2) // [1,2]

// map (key value pair)
func main() {
    // var x string
    // x = "hello world"
    // y := "goodbye world"
    // fmt.Println(x)
    // fmt.Println(y)
    // fmt.Println(x == "hello")
    // fmt.Println(x == ("hello " + "world"))

    // map variable declaration & initialization
    var x map[string]int
    x = map[string]int{
        "a": 1,
    }
    x["key"] = 10
    fmt.Println(x)

    elements := map[string]string{
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
    }
    elements["key10"] = "value10"
    delete(elements, "key1")
    fmt.Println(elements)

    name, ok := elements["key2"]
    fmt.Println(name, ok)

    if name, ok := elements["key1"]; ok {
        fmt.Println(name, ok)
    }

    y := make(map[string]string)
    y["key1000"] = "1000"
    fmt.Println(y)
}
```

### make() and new()

`make()` is used for follows.

- Create a channel
- Create a map with space preallocated
- Create a slice with space preallocated or with len != cap

It's a little harder to justify `new()`. The main thing it makes easier is creating pointers to non-composite types. The two functions below are equivalent. One's just a little more concise:

```go
func newInt1() *int { return new(int) }

func newInt2() *int {
    var i int
    return &i
}
```

### function

```go
func add(x int, y int) int {
    return x + y
}

// named return variables
func f2() (r int) {
    r = 1
    return
}

// multiple return variables
func f() (int, int) {
    return 5, 6
}

func main() {
    x, y := f()
}

// 가변함수 Variadic Functions (print함수도 가변함수)
// func Println(a ...interface{}) (n int, err error)
func add(args ...int) int {
    total := 0
    for _, v := range args {
         total += v
    }
    return total
}

func main() {
    fmt.Println(add(1,2,3))
}

// ...를 이용해 slice를 argument로 전달
func main() {
    xs := []int{1,2,3}
    fmt.Println(add(xs...))
}

// Closures (a function in a function)
func intSeq() func() int {
    i := 0
    return func() int {
        i++
        return i
    }
}

func main() {
    nextInt := intSeq()
    fmt.Println(nextInt())
    fmt.Println(nextInt())
    fmt.Println(nextInt())

    newInts := intSeq()
    fmt.Println(newInts())
}

// Recursive function
func factorial(x uint) uint {
    if x == 0 {
        return 1
    }
    return x * factorial(x-1)
}

// defer (invoked at the end of the block) function
func first() {
    fmt.Println("1st")
}
func second() {
    fmt.Println("2nd")
}
func main() {
    defer second()
    first()
}

// 일반적으로 함수 종료시 자원을 해제할 때 사용
// runtime panic이 발생하더라도 실행
f, _ := os.Open(filename)
defer f.Close()

// panic and recover functions
// recover는 defer와 사용해야 함.
func main() {
    defer func() {
        // recover function
        str := recover()
        fmt.Println("recovery: ", str)
    }()
    panic("Panic !! string")
}
```

### Pointers

go는 garbage collection을 지원하므로 point 해제 (free)가 필요없음.

```go
func zero(xPtr *int) {
    *xPtr = 0
}
func main() {
    x := 5
    zero(&x)
    fmt.Println(x) // x는 0
}

// point using new()
func one(xPtr *int) {
   *xPtr = 1
}
func main() {
    xPtr := new(int)
    one(xPtr)
    fmt.Println(*xPtr) // x는 1
}
```

### Structure (field and method)

- Field가 대문자로 시작시 C++의 public의 특성을 가짐 (외부 package에서 참조 가능)
- Field가 소문자로 시작시 C++의 private의 특성을 가짐 (외부 package에서 참조 불가능)
- 아래 예제에서 X, Y, Z는 외부 package에서 참조 가능
```go

// Clircle blarara o
type Circle struct {
    x float64
    y float64
    r float64
    
    X float64
    Y float64
    R float64
}

// call by value function
func circleArea(c Circle) float64 {
    return math.Pi * (c.r * c.r)
}

// call by reference function
func circleArea2(c *Circle) float64 {
    return math.Pi * (c.r * c.r)
}

// method (define a pointer receiver in parenthesis)
func (c *Circle) area() float64 {
    return math.Pi * c.r * c.r
}

// method (define a value receiver in parenthesis)
func (c Circle) area2() float64 {
    return math.Pi * c.r * c.r
}

func main() {
    var a Circle
    fmt.Println(a)
    var b Circle = Circle{x: 1, y: 2, r: 3}
    fmt.Println(b)
    c := Circle{10, 20, 30}
    fmt.Println(c)

    // Circle pointer
    d := new(Circle)
    d.x = 20
    d.y = 30
    d.r = 10
    fmt.Println(d)

    // call by value (copy Circle argument)
    fmt.Println(circleArea(c))
    fmt.Println(circleArea(*d))

    // call by reference
    fmt.Println(circleArea2(d))

    // method
    fmt.Println("method:", b.area())
}
```

### Anonymous fields in Go Structure

- Type이름으로 field가 생성됨
- Anonymous field로 정의된 structure의 field 및 method를 선언된 structure가 상속함.

```go
type Kitchen struct {
    numOfPlates int
}

type House struct {
    Kitchen    // anonymous field
    numOfRooms int
}

func main() {
    //to initialize you have to use composed type name.
    h := House{Kitchen{10}, 3}
    //numOfRooms is a field of House
    fmt.Println("House h has this many rooms:", h.numOfRooms)
    //numOfPlates is a field of anonymous field Kitchen, so it can be referred to like a field of House
    fmt.Println("House h has this many plates:", h.numOfPlates)
    //we can refer to the embedded struct in its entirety by referring to the name of the struct type
    fmt.Println("The Kitchen contents of this house are:", h.Kitchen)
}
```

### Interface (collections of methods)

- Interfaces are named collections of method signatures.
- 구조체(struct)가 필드들의 집합체라면, interface는 메서드들의 집합체
- interface는 타입(type)이 구현해야 하는 메서드 원형(prototype)들을 정의함.
- type의 interface를 구현하기 위해서는 단순히 그 인터페이스가 갖는 모든 메서드들을 구현하면 됨.
- Interface 변수는 1) interface type과 2) 실제 가리키는 data 주소로 이루어짐


![interface 구조](https://miro.medium.com/max/700/1*nLosvZkY_o-ltxnUNsRyqQ.png)

```go
// Shape interface type definition
type Shape interface {
    area() float64
    perimeter() float64
}

//Rect 정의
type Rect struct {
    width, height float64
}

//Circle 정의
type Circle struct {
    radius float64
}

//Rect 타입에 대한 Shape 인터페이스 구현
func (r Rect) area() float64 { return r.width * r.height }
func (r Rect) perimeter() float64 {
    return 2 * (r.width + r.height)
}

//Circle 타입에 대한 Shape 인터페이스 구현
func (c Circle) area() float64 {
    return math.Pi * c.radius * c.radius
}
func (c Circle) perimeter() float64 {
    return 2 * math.Pi * c.radius
}

func main() {
    r := Rect{10., 20.}
    c := Circle{10}

    showArea(r, c)
}

func showArea(shapes ...Shape) {
    for _, s := range shapes {
        a := s.area() //인터페이스 메서드 호출
        println(a)
    }
}

// interface field in structure
// [FIXME] how to ?
```

```go
//  example 2.
type Abser interface {
    Abs() float64
}

func main() {
    var a Abser
    f := MyFloat(-math.Sqrt2)
    v := Vertex{3, 4}

    a = f // a MyFloat implements Abser
    fmt.Println(a.Abs())
    a = &v // a *Vertex implements Abser

    fmt.Println(a.Abs())
}

type MyFloat float64

func (f MyFloat) Abs() float64 {
    if f < 0 {
        return float64(-f)
    }
    return float64(f)
}

type Vertex struct {
    X, Y float64
}

func (v *Vertex) Abs() float64 {
    return math.Sqrt(v.X*v.X + v.Y*v.Y)
}
```

### empty interface

- 어떠한 Type도 담을 수 interface

![empty interface](https://miro.medium.com/max/513/1*EQuV1IIhITb12L2As1QndA.png)



```go
func Marshal(v interface{}) ([]byte, error);
func Println(a ...interface{}) (n int, err error);
```

```go
func main() {
    var i interface{}
    describe(i)

    i = 42
    describe(i)

    i = "hello"
    describe(i)
}

func describe(i interface{}) {
    fmt.Printf("(%v, %T)\n", i, i)
}
// (<nil>, <nil>)
// (42, int)
// (hello, string)
```

### Errors

- By convention, errors are the last return value and have type error, a built-in interface.
- `errors.New` constructs a basic error value with the given error message.
- A `nil` value in the error position indicates that there was **no error**.

```go
func f1(arg int) (int, error) {
    if arg == 42 {
        return -1, errors.New("can't work with 42")
    }
    return arg + 3, nil
}

func main() {
    for _, i := range []int{7, 42} {
        if r, e := f1(i); e != nil {
            fmt.Println("f1 failed:", e)
        } else {
            fmt.Println("f1 worked:", r)
        }
    }
}
```

### Concurrency (goroutine)

```go
// goroutine to support concurrency
package main
import (
    "fmt"
    "time"
)

func f(from string) {
    for i := 0; i < 3; i++ {
        fmt.Println(from, ":", i)
    }
}

func main() {

    f("direct")

    go f("goroutine")

    go func(msg string) {
        fmt.Println(msg)
    }("going")

    time.Sleep(time.Second)
    fmt.Println("done")
}
```

### Channel

Channels are the pipes that connect concurrent goroutines. You can send values into channels from one goroutine and receive those values into another goroutine.

```go
package main

import "fmt"

func main() {

    messages := make(chan string)

    go func() { messages <- "ping" }()

    msg := <-messages
    fmt.Println(msg)
}
```

```go
# Channel direction
# channel의 방향을 지정가능함.

func pinger(c chan<- string)
func printer(c <-chan string)
```

### Channel Buffering

By default channels are unbuffered, meaning that they will only accept sends (chan <-) if there is a corresponding receive (<- chan) ready to receive the sent value.

```go
package main

import "fmt"

func main() {

    messages := make(chan string, 2)

    messages <- "buffered"
    messages <- "channel"

    fmt.Println(<-messages)
    fmt.Println(<-messages)
}
```

### Channel Synchronization (Waitgroup)

```go
package main

import (
    "fmt"
    "time"
)

func worker(done chan bool) {
    fmt.Print("working...")
    time.Sleep(time.Second)
    fmt.Println("done")
    done <- true
}

func main() {
    done := make(chan bool, 1)
    go worker(done)
    <-done
}
```

```go
// goroutine with WaitGroup
package main

import "sync"
import "fmt"
import "time"

type Object struct {
    //data
}

func (obj *Object) Update(wg *sync.WaitGroup) {
    //update data
    time.Sleep(time.Second)
    fmt.Println("Update done")
    wg.Done()
    return
}

func main() {
    var wg sync.WaitGroup
    list := make([]Object, 5)
    for {
        for _, object := range list {
            wg.Add(1)
            go object.Update(&wg)
        }
        //now everything has been updated. start again
        wg.Wait()
        fmt.Println("Group done")
    }
}
```

### Select (for Channel)

Go’s select lets you wait on multiple channel operations. Combining goroutines and channels with select is a powerful feature of Go.

```go
package main

import (
    "fmt"
    "time"
)

func main() {
    c1 := make(chan string)
    c2 := make(chan string)

    go func() {
        for {
            c1 <- "from 1"
            time.Sleep(time.Second * 2)
        }
    }()
    go func() {
        for {
            c2 <- "from 2"
            time.Sleep(time.Second * 3)
        }
    }()
    go func() {
        for {
            select {
            case msg1 := <-c1:
                fmt.Println(msg1)
            case msg2 := <-c2:
                fmt.Println(msg2)
            case current := <-time.After(time.Second):
                fmt.Println(current)
            }
        }
    }()

    var input string
    fmt.Scanln(&input)
}
```

### context

A way to think about context package in go is that it allows you to pass in a “context” to your program. Context like a timeout or deadline or a channel to indicate stop working and return.

- https://jaehue.github.io/post/how-to-use-golang-context/
- http://p.agnihotry.com/post/understanding_the_context_package_in_golang/

```go
func longFunc() string {
    <-time.After(time.Second * 3) // long running job
    return "Success"
}

func longFuncWithCtx(ctx context.Context) (string, error) {
    done := make(chan string)

    go func() {
        done <- longFunc()
    }()

    select {
    case result := <-done:
        return result, nil
    case <-ctx.Done():
        return "Fail", ctx.Err()
    }
}
```

### cgo

```go
package main

/*
#include <stdio.h>

extern int sum(int a, int b); // Go 언어의 함수는 extern으로 선언

static inline void CExample() {
    int r = sum(1, 2); // Go 언어의 sum 함수 호출
    printf("%d\n", r);
}
*/
import "C"

//export sum
func sum(a, b C.int) C.int { // C 언어에서 사용할 수 있도록 매개변수와 리턴값 자료형을
                             // C 언어용으로 맞춰줌
    return a + b
}

func main() {
    C.CExample()
}
```

```go
package test

/*
#include <stdio.h>

int sum(int a, int b) // 덧셈 함수 작성
{
    return a + b;
}

void hello() // Hello, world! 출력 함수 작성
{
    printf("Hello, world!\n");
}
*/
import "C"
import "fmt"

func main() {
    var a, b int = 1, 2
    r := C.sum(C.int(a), C.int(b)) // C 언어 함수 sum 호출
    fmt.Println(r)                 // 3

    C.hello() // Hello, world!
}
```

### Command-Line

The way to get program start argument and options.

```go

// Command-Line Arguments and Flags
package main

import (
    "fmt"
    "os"
)

func main() {
    argWithName := os.Args
    argWithoutName := os.Args[1:]
    arg := os.Args[3]

    fmt.Println(argWithName)
    fmt.Println(argWithoutName)
    fmt.Println(arg)

    wordPtr := flag.String("word", "default", "a string option")
    numbPtr := flag.Int("num", 1, "integer option")
    boolPtr := flag.Bool("bool", false, "boolean option")
    var sval string
    flag.StringVar(&sval, "str-var", "default", "string variable option")
    flag.Parse()
    fmt.Println("word:", *wordPtr)
    fmt.Println("num:", *numbPtr)
    fmt.Println("bool:", *boolPtr)
    fmt.Println("str-var:", sval)
    fmt.Println("tail:", flag.Args())
}

// Command-Line subcommands
package main

import (
    "flag"
    "fmt"
    "os"
)

func main() {

    fooCmd := flag.NewFlagSet("foo", flag.ExitOnError)
    fooEnable := fooCmd.Bool("enable", false, "enable")
    fooName := fooCmd.String("name", "", "name")

    barCmd := flag.NewFlagSet("bar", flag.ExitOnError)
    barLevel := barCmd.Int("level", 0, "level")

    if len(os.Args) < 2 {
        fmt.Println("expected 'foo' or 'bar' subcommands")
        os.Exit(1)
    }

    switch os.Args[1] {

    case "foo":
        fooCmd.Parse(os.Args[2:])
        fmt.Println("subcommand 'foo'")
        fmt.Println("  enable:", *fooEnable)
        fmt.Println("  name:", *fooName)
        fmt.Println("  tail:", fooCmd.Args())
    case "bar":
        barCmd.Parse(os.Args[2:])
        fmt.Println("subcommand 'bar'")
        fmt.Println("  level:", *barLevel)
        fmt.Println("  tail:", barCmd.Args())
    default:
        fmt.Println("expected 'foo' or 'bar' subcommands")
        os.Exit(1)
    }
}

// usage

package main

import (
    "flag"
    "fmt"
    "os"
)

func main() {
    flag.Usage = func() {
        fmt.Printf("Usage of %s:\n", os.Args[0])
        fmt.Printf("    example7 file1 file2 ...\n")
        flag.PrintDefaults()
    }
    flag.Parse()
}
```


> https://www.digitalocean.com/community/tutorials/how-to-use-the-flag-package-in-go

### empty structure

```go
package main

import "fmt"

type Duck struct {
}

func (d Duck) quack() {
    fmt.Println("Quack~")
}

func (d Duck) feathers() {
    fmt.Println("White and gray feathers")
}

type Person struct {
}

func (p Person) quack() {
    fmt.Println("No Quack~")
}

func (p Person) feathers() {
    fmt.Println("no feathers")
}

type Quacker interface {
    quack()
    feathers()
}

func inTheForest(q ...Quacker) {
    for _, entry := range q {
        fmt.Printf("%T\n", entry)
        entry.feathers()
        entry.quack()
    }
}

func main() {
    var donald Duck
    var john Person
    inTheForest(donald, john)
}

```

### structTag

A StructTag is the tag string in a struct field.
By convention, tag strings are a concatenation of optionally space-separated key:"value" pairs. Each key is a non-empty string consisting of non-control characters other than space (U+0020 ' '), quote (U+0022 '"'), and colon (U+003A ':'). Each value is quoted using U+0022 '"' characters and Go string literal syntax.

- https://golang.org/pkg/reflect/#StructTag
- https://golangbot.com/reflection/

> Format: `type StructTag string`

```go
package main

import (
    "fmt"
    "reflect"
)

func main() {
    type S struct {
        F string `species:"gopher" color:"blue"`
    }

    s := S{}
    st := reflect.TypeOf(s)
    field := st.Field(0)
    fmt.Println(field.Tag.Get("color"), field.Tag.Get("species"))
}
```

### reflection

```go

type Person struct {
    name string `tag1:"1" tag2:"2"`
    age  int    `tag1:"나이" tag2:"Age"`
}

func main() {
    var i int = 1
    var s string = "hello world"
    var f float32 = 1.3

    fmt.Println(reflect.TypeOf(i))
    fmt.Println(reflect.TypeOf(s))
    fmt.Println(reflect.TypeOf(f))
    t := reflect.TypeOf(f)
    v := reflect.ValueOf(f)
    fmt.Println("")
    fmt.Println("float32 reflection")
    fmt.Println("==================")
    fmt.Println(t.Name())
    fmt.Println(t.Size())
    fmt.Println(t.Kind() == reflect.Float32)
    fmt.Println(t.Kind() == reflect.Float64)
    fmt.Println(v.Type())
    fmt.Println(v.Kind() == reflect.Float32)
    fmt.Println(v.Kind() == reflect.Float64)
    fmt.Println(v.Float())
    fmt.Println(v)

    fmt.Println("")
    fmt.Println("struct reflection")
    fmt.Println("==================")
    var d Person = Person{"myname", 3}
    var p *Person = &d
    fmt.Println(reflect.TypeOf(d))
    name, ok := reflect.TypeOf(d).FieldByName("name")
    fmt.Println("num of fields", reflect.TypeOf(d).NumField())
    fmt.Println(ok, name.Tag.Get("tag1"), name.Tag.Get("tag2"))
    age, ok := reflect.TypeOf(d).FieldByName("age")
    fmt.Println(ok, age.Tag.Get("tag1"), age.Tag.Get("tag2"))
    fmt.Println(reflect.TypeOf(p))
    fmt.Println(reflect.ValueOf(p))
    fmt.Println(reflect.ValueOf(p).Elem()) // reflection of pointer
    fmt.Println(reflect.ValueOf(p).Elem().FieldByName("name"),
        reflect.ValueOf(p).Elem().FieldByName("age"))

    fmt.Println("")
    fmt.Println("interface reflection")
    fmt.Println("==================")
    var b interface{}
    b = 1
    fmt.Println(reflect.TypeOf(b))
    fmt.Println(reflect.ValueOf(b))
    fmt.Println(reflect.ValueOf(b).Int())
    // fmt.Println(reflect.ValueOf(b).Elem()) // Runtime error
}
```

### Type assertions

A type assertion doesn’t really convert an interface to another data type, but it provides access to an interface’s concrete value, which is typically what you want.

The type assertion x.(T) asserts that the concrete value stored in x is of type T, and that x is not nil.

- If T is not an interface, it asserts that the dynamic type of x is identical to T.
- If T is an interface, it asserts that the dynamic type of x implements T.

```go
var x interface{} = "foo"

var s string = x.(string)
fmt.Println(s)     // "foo"

s, ok := x.(string)
fmt.Println(s, ok) // "foo true"

n, ok := x.(int)
fmt.Println(n, ok) // "0 false"

n = x.(int)        // ILLEGAL
```

### Type switches

type switch performs several type assertions in series and runs the first case with a matching type.

```go
var x interface{} = "foo"

switch v := x.(type) {
case nil:
    fmt.Println("x is nil")            // here v has type interface{}
case int:
    fmt.Println("x is", v)             // here v has type int
case bool, string:
    fmt.Println("x is bool or string") // here v has type interface{}
default:
    fmt.Println("type unknown")        // here v has type interface{}
}
```

## cgo (golang with C)

```go
package main

// typedef int (*intFunc) ();
//
// int
// bridge_int_func(intFunc f)
// {
//      return f();
// }
//
// int fortytwo()
// {
//      return 42;
// }
import "C"
import "fmt"

func main() {
    f := C.intFunc(C.fortytwo)
    fmt.Println(int(C.bridge_int_func(f)))
    // Output: 42
}
```

> https://eli.thegreenplace.net/2019/passing-callbacks-and-pointers-to-cgo/

## go with gRPC

```bash
go get google.golang.org/grpc
go get -u github.com/golang/protobuf/protoc-gen-go
<!-- plugins: grpc interface 생성함. -->
protoc -I <PATH_TO_INCLUDE> --go_out=plugins=grpc:<PATH_TO_GENERATE> <PATH_TO_PROTO_FILE>
protoc -I <PATH_TO_INCLUDE> --go_out=noti <PATH_TO_PROTO_FILE>
```

## Useful commands

```bash
# show go doc on web
go get -v golang.org/x/tools/cmd/godoc
godoc fmt.Println

# build package
cd $GOPATH/src
go build <TARGET_DIR=PACKAGE> # test build
go install <TARGET_DIR=PACKAGE> # copy *.so to $GOPATH/pkg

```

## Useful functions

### panic() and recover()

They Generate an runtime error and recover (receive) the runtime error.

### os.Exit()

Use os.Exit to immediately exit with a given status. `defer`s will not be run when using os.Exit, so this fmt.Println will never be called.

## Useful documents

- https://github.com/golang-kr/golang-doc/wiki/Go-%EC%BD%94%EB%93%9C%EB%A5%BC-%EC%9E%91%EC%84%B1%ED%95%98%EB%8A%94-%EB%B0%A9%EB%B2%95
- https://golang.org/doc/effective_go.html

## gRPC with simple password authentication

```go
// client
grpc.Dial(target,
    grpc.WithInsecure(),
    grpc.WithPerRPCCredentials(&loginCreds{
    Username: "admin",
    Password: "admin123",
}))

type loginCreds struct {
    Username, Password string
}

func (c *loginCreds) GetRequestMetadata(context.Context, ...string) (map[string]string, error) {
    return map[string]string{
        "username": c.Username,
        "password": c.Password,
    }, nil
}

func (c *loginCreds) RequireTransportSecurity() bool {
    return true
}

// server
grpc.NewServer(
    grpc.StreamInterceptor(streamInterceptor), 
    grpc.UnaryInterceptor(unaryInterceptor)
)

func streamInterceptor(srv interface{}, stream grpc.ServerStream, info *grpc.StreamServerInfo, handler grpc.StreamHandler) error {
    if err := authorize(stream.Context()); err != nil {
        return err
    }

    return handler(srv, stream)
}

func unaryInterceptor(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
    if err := authorize(ctx); err != nil {
        return err
    }

    return handler(ctx, req)
}

func authorize(ctx context.Context) error {
    if md, ok := metadata.FromContext(ctx); ok {
        if len(md["username"]) > 0 && md["username"][0] == "admin" &&
            len(md["password"]) > 0 && md["password"][0] == "admin123" {
            return nil
        }

        return AccessDeniedErr
    }

    return EmptyMetadataErr
}
```


## vscode with golang

> https://github.com/Microsoft/vscode-go/issues/441

> https://rominirani.com/setup-go-development-environment-with-visual-studio-code-7ea5d643a51a

```shell
go get -u github.com/nsf/gocode
```

## Go Fuzzing

Fuzzing is a type of automated testing which continuously manipulates inputs to a program to find bugs.

> 임의의 값을 입력하여 functional test를 수행하는 test framework이라 볼 수 있음.

![Fuzzing Test](https://go.dev/doc/fuzz/example.png)

