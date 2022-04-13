# javascript

이 문서는 아래 javascript tutorial을 보고 정리한 것임

📁 https://ko.javascript.info

> 자바스크립트의 공식이름?: `ECMAScript`

- [javascript](#javascript)
  - [Javascript engine](#javascript-engine)
  - [Garbage Collection](#garbage-collection)
  - [javascript framework](#javascript-framework)
  - [Linter](#linter)
  - [JS 문서화](#js-문서화)
  - [Polyfill](#polyfill)
  - [테스트 자동화](#테스트-자동화)
  - [Comment 주석](#comment-주석)
  - [use strict](#use-strict)
  - [브라우저 콘솔](#브라우저-콘솔)
  - [전역 객체](#전역-객체)
    - [Polyfill의 구현](#polyfill의-구현)
  - [변수와 상수](#변수와-상수)
    - [변수 명명 규칙](#변수-명명-규칙)
    - [함수형 언어](#함수형-언어)
    - [즉시 실행 함수 표현식](#즉시-실행-함수-표현식)
  - [자료형](#자료형)
    - [숫자형](#숫자형)
      - [isNaN, isFinite](#isnan-isfinite)
      - [parseInt, parseFloat](#parseint-parsefloat)
      - [Math](#math)
    - [문자형](#문자형)
    - [`boolean`형](#boolean형)
    - [`null` 값](#null-값)
    - [`undefined` 값](#undefined-값)
    - [`object`](#object)
      - [const object의 내부 정보 변경 가능](#const-object의-내부-정보-변경-가능)
      - [예약어 키로 사용 - ok](#예약어-키로-사용---ok)
      - [키에 숫자 0을 넣으면 문자열 "0"으로 자동변환](#키에-숫자-0을-넣으면-문자열-0으로-자동변환)
      - [object 비교](#object-비교)
      - [객체 복사, 병합과 Object.assign](#객체-복사-병합과-objectassign)
      - [Methods and this](#methods-and-this)
      - [객체 생성 함수 constructor function](#객체-생성-함수-constructor-function)
      - [옵셔널 체이닝(optional chaining) `?.`, `?.()`, `?.[]`](#옵셔널-체이닝optional-chaining---)
      - [object 순회 함수](#object-순회-함수)
    - [`symbol`](#symbol)
      - [객체의 형변환; Symbol.toPrimitive](#객체의-형변환-symboltoprimitive)
      - [iterable 객체 만들기; Symbol.iterator](#iterable-객체-만들기-symboliterator)
    - [`Array`](#array)
      - [mapping function for array](#mapping-function-for-array)
      - [sort](#sort)
      - [문자열 <==> 배열](#문자열--배열)
      - [reduce와 reduceRight (누산기)](#reduce와-reduceright-누산기)
      - [배열인지 확인](#배열인지-확인)
    - [`map`](#map)
      - [제공 함수](#제공-함수)
    - [`set`](#set)
    - [`WeakMap` and `WeakSet`](#weakmap-and-weakset)
    - [`Date`](#date)
      - [Autocorrection](#autocorrection)
      - [Benchmarking Test](#benchmarking-test)
      - [Date.parse](#dateparse)
  - [JSON (JavaScript Object Notation) Serialization](#json-javascript-object-notation-serialization)
    - [`toJSON()` for customized serialization](#tojson-for-customized-serialization)
  - [`typeof`](#typeof)
  - [Browser functions](#browser-functions)
  - [형변환 (type conversion)](#형변환-type-conversion)
  - [연산자](#연산자)
    - [연산자 우선순위 Operator precedence table](#연산자-우선순위-operator-precedence-table)
    - [할당 연산자](#할당-연산자)
    - [쉼표 연산자](#쉼표-연산자)
  - [비교 연산자](#비교-연산자)
    - [error 비교](#error-비교)
    - [문자열 비교](#문자열-비교)
    - [다른형간의 비교](#다른형간의-비교)
    - [일치 연산자(strict equality operator) ===](#일치-연산자strict-equality-operator-)
    - [null이나 undefined와 비교하기](#null이나-undefined와-비교하기)
    - [단락 평가 - short circuit evaluation](#단락-평가---short-circuit-evaluation)
    - [nullish 병합 연산자 (nullish coalescing operator) `??`](#nullish-병합-연산자-nullish-coalescing-operator-)
  - [조건문](#조건문)
  - [반복문](#반복문)
  - [switch/case문](#switchcase문)
  - [Function](#function)
    - [Function Expression (함수 표현식)](#function-expression-함수-표현식)
    - [Arrow function (화살표 함수)](#arrow-function-화살표-함수)
    - [debugger](#debugger)
    - [console.log](#consolelog)
    - [Function property](#function-property)
    - [variable arguments](#variable-arguments)
    - [Input argument spreading](#input-argument-spreading)
    - [Closure and Lexical scoping 클로저와 어휘 범위 지정](#closure-and-lexical-scoping-클로저와-어휘-범위-지정)
    - [new Function](#new-function)
  - [Time scheduling - `setTimeout`, `setInterval`](#time-scheduling---settimeout-setinterval)
  - [Testing - Mocha](#testing---mocha)
    - [chai (assertion logic)](#chai-assertion-logic)
  - [구조 분해 할당 (destructuring assignment)](#구조-분해-할당-destructuring-assignment)
    - [배열 분해 할당](#배열-분해-할당)
    - [객체 분해 할당](#객체-분해-할당)
    - [중첩 구조 분해(nested destructuring)](#중첩-구조-분해nested-destructuring)
    - [function argument with destructuring assignment](#function-argument-with-destructuring-assignment)
  - [`Promise`](#promise)
    - [`.then`](#then)
    - [`.catch`](#catch)
    - [`.finally`](#finally)
    - [Promise chaining](#promise-chaining)
    - [`thenable`](#thenable)
    - [`fetch`](#fetch)
    - [Throw an error in promise](#throw-an-error-in-promise)
    - [Mutilple promises](#mutilple-promises)
    - [promisify](#promisify)
    - [Microtask queue](#microtask-queue)
  - [`Async` and `Await`](#async-and-await)
    - [async 클래스 메서드](#async-클래스-메서드)
    - [`async` & `await` error handling](#async--await-error-handling)
  - [File Read/Write](#file-readwrite)
  - [Decorator (wrapping function)](#decorator-wrapping-function)
    - [객체의 `call`함수 사용하기](#객체의-call함수-사용하기)
  - [함수의 호출 함수](#함수의-호출-함수)
  - [object property 속성 설정](#object-property-속성-설정)
  - [getter, setter properties](#getter-setter-properties)
  - [nodejs](#nodejs)
    - [import package](#import-package)
    - [nodejs getting start](#nodejs-getting-start)

## Javascript engine

- `V8`
- `Firefox`?

## Garbage Collection

- reachability 도달 가능성이 없는 변수는 삭제
- mark-and-sweep
  - generational collection: 오랜시간 남은 개체 덜 감시
  - incremental collection: 확인 개체 grouping하고 분산 처리
  - idle-time collection: 유휴 시간 실행

## javascript framework

- `jQuery`
- `Lodash`

## Linter

- `JSLint` – 역사가 오래된 linter
- `JSHint` – JSLint보다 세팅이 좀 더 유연한 linter
- `ESLint` – 가장 최근에 나온 linter, `npm install -g eslint`

## JS 문서화

- `JSDoc`: http://usejsdoc.org/

```javascript
/**
 * x를 n번 곱한 수를 반환함
 *
 * @param {number} x 거듭제곱할 숫자
 * @param {number} n 곱할 횟수, 반드시 자연수여야 함
 * @return {number} x의 n 거듭제곱을 반환함
 */
function pow(x, n) {
  ...
}
```

## Polyfill

폴리필(Polyfill)이란 브라우저가 지원하지 않는 자바스크립트 코드를 지원 가능하도록 변환한 코드
하위 브라우저가 지원하는 자바스크립트 코드를 사용해 자바스크립트의 최신 기능을 똑같이 구현한 library?

- 트랜스파일러: 바벨은 개발자의 컴퓨터에서 돌아가는데, 이를 실행하면 기존 코드가 구 표준을 준수하는 코드로 변경되어 웹을 통해 load됨, 웹팩(webpack)
- 폴리필: 생략

주목할 만한 폴리필 두 가지는 아래와 같습니다.

- `core js` – 다양한 폴리필을 제공합니다. 특정 기능의 폴리필만 사용하는 것도 가능합니다.
- `polyfill.io` – 기능이나 사용자의 브라우저에 따라 폴리필 스크립트를 제공해주는 서비스입니다.
- `Babel`

## 테스트 자동화

- `Mocha`: describe, it과 같은 테스팅 함수와 테스트 실행 관련 주요 함수를 제공, https://mochajs.org/
- `Chai` – 다양한 assertion을 제공해 주는 라이브러리
- `Sinon` – 함수의 정보를 캐내는 데 사용되는 라이브러리로, 내장 함수 등을 모방

FIXME: https://ko.javascript.info/testing-mocha

## Comment 주석

`//`, `/* */`

## use strict

- `ECMAScript5(ES5)` 기본 문법 일부 

```javascript
"use strict"
// ES5 모드로 동작
```

## 브라우저 콘솔

개발한 기능을 테스트하기 위해 브라우저 콘솔을 사용하는 경우, 기본적으로 use strict가 적용되어 있지 않음

## 전역 객체

브라우저 환경에선 전역 객체를 `window`, Node.js 환경에선 `global` (`globalThis`)

```javascript
var gVar = 5;
// var로 선언한 변수는 전역 객체 property)
alert(window.gVar); // 5
```

### Polyfill의 구현 

```javascript
if (!window.Promise) {
  alert("구식 브라우저를 사용 중이시군요!");
}
if (!window.Promise) {
  window.Promise = ... // 모던 자바스크립트에서 지원하는 기능을 직접 구현함
}
```


## 변수와 상수

- javascript primitive variables: 문자(string), 숫자(number), bigint, 불린(boolean), 심볼(symbol), null, undefined형
- javascript의 변수는 값 수정이 아니라 교체되는 개념

```javascript
let message;

message = 'Hello'; // 문자열을 저장합니다.
alert(message); // 변수에 저장된 값을 보여줍니다.

// 선언 방법
let user = 'John', age = 25, message = 'Hello';

let user = 'John';
let age = 25;
let message = 'Hello';

let user = 'John',
  age = 25,
  message = 'Hello';

// var는 let과 거의 동일하나, local에서 선언하더라도 global
var message = 'Hello';

// 상수 선언; 한번 할당되면 변경 불가
const myBirthday = '18.04.1982';
myBirthday = '01.01.2001'; // error, can't reassign the constant!

// 대문자 상수 (일반적 관습)
const COLOR_RED = "#F00";
const COLOR_GREEN = "#0F0";

// 상수 사용
let color = COLOR_ORANGE;
alert(color); // #FF7F00

// 아래 상수는 값 할당 전이므로 이후 할당 가능
// 할당후에는 변경 불가
const pageLoadTime = ;

```

### 변수 명명 규칙

- `문자`와 `숫자`, 그리고 기호 `$`와 `_` 사용
- 카멜 표기법(camelCase) 주로 사용
- 예약어(reserved name) 사용 X
- 첫단어 `숫자` X
- 대소문자 구별


### 함수형 언어

- 함수형(functional) 프로그래밍 언어는 변숫값 변경을 금지
- e.g. 스칼라(Scala)와 얼랭(Erlang)
- 병렬 계산(parallel computation)에 유용

### 즉시 실행 함수 표현식

즉시 실행 함수 표현식(immediately-invoked function expressions): 선언과 함께 함수 실행

```javascript
// IIFE를 만드는 방법

(function() {
  alert("함수를 괄호로 둘러싸기");
})();

(function() {
  alert("전체를 괄호로 둘러싸기");
}());

!function() {
  alert("표현식 앞에 비트 NOT 연산자 붙이기");
}();

+function() {
  alert("표현식 앞에 단항 덧셈 연산자 붙이기");
}();
```

## 자료형

### 숫자형

- 숫자형 범위: `(2^53-1)` ~ `-(2^53-1)`; `9007199254740991`
- `BigInt`: 끝에 `n`을 붙이면 `BigInt`
- `Infinity` : 무한대, `1/0` 로도 구해짐
- `-Infinity`
- `NaN`: 계산 오류, e.g. `"STR"/2`, 수로 문자를 나눌 경우

```javascript
let n = 123;
n = 12.345;

// 끝에 'n'이 붙으면 BigInt형 자료입니다.
const bigInt = 1234567890123456789012345678901234567890n;

let billion = 1000000000;
let billion = 1e9;  // 10억, 1과 9개의 0
alert( 7.3e9 );  // 73억 (7,300,000,000)
1e3 = 1 * 1000
1.23e6 = 1.23 * 1000000
let ms = 0.000001;
let ms = 1e-6; // 1에서 왼쪽으로 6번 소수점 이동
// 10을 세 번 거듭제곱한 수로 나눔
1e-3 = 1 / 1000 (=0.001)
// 10을 여섯 번 거듭제곱한 수로 나눔
1.23e-6 = 1.23 / 1000000 (=0.00000123)

// 16, 8, 2진수표현
alert( 0xff ); // 255
alert( 0xFF ); // 255 (대·소문자를 가리지 않으므로 둘 다 같은 값을 나타냅니다.)
let a = 0b11111111; // 255의 2진수
let b = 0o377; // 255의 8진수
alert( a == b ); // true, 진법은 다르지만, a와 b는 같은 수임
let num = 255;
alert( num.toString(16) );  // ff
alert( num.toString(2) );   // 11111111
alert( 123456..toString(36) ); // 2n9c // base36

// Rounding 어림수
Math.floor // 소수점 첫번째 자리 버림
Math.ceil // 소수점 올림
Math.round // 반올림
// https://ko.javascript.info/number

//  숫자가 너무 커지면 64비트 공간이 넘쳐서 Infinity
alert( 1e500 ); // Infinity
// 정밀도 손실
alert( 0.1 + 0.2 == 0.3 ); // false
// 정밀도 손실 확인 toFixed(n), n은 소수점 자릿수에서 어림
alert( 0.1.toFixed(20) ); // 0.10000000000000000555
let sum = 0.1 + 0.2;
alert( sum.toFixed(2) ); // 0.30
```

#### isNaN, isFinite

```javascript
// NaN은 NaN 자기 자신을 포함하여 그 어떤 값과도 같지 않다는 점에서 독특합니다.
alert( NaN === NaN ); // false

alert( isNaN(NaN) ); // true
alert( isNaN("str") ); // true

alert( isFinite("15") ); // true
alert( isFinite("str") ); // false, NaN이기 때문입니다.
alert( isFinite(Infinity) ); // false, Infinity이기 때문입니다.

```

#### parseInt, parseFloat

```javascript
alert( parseInt('100px') ); // 100
alert( parseFloat('12.5em') ); // 12.5
alert( parseInt('12.3') ); // 12, 정수 부분만 반환됩니다.
alert( parseFloat('12.3.4') ); // 12.3, 두 번째 점에서 숫자 읽기를 멈춥니다.
alert( parseInt('a123') ); // NaN, a는 숫자가 아니므로 숫자를 읽는 게 중지됩니다.
alert( parseInt('0xff', 16) ); // 255
alert( parseInt('ff', 16) ); // 255, 0x가 없어도 동작합니다.
alert( parseInt('2n9c', 36) ); // 123456
```

#### Math

```javascript
alert( Math.random() ); // 0.1234567894322
alert( Math.random() ); // 0.5435252343232
alert( Math.random() ); // ... (무작위 수)
alert( Math.max(3, 5, -10, 0, 1) ); // 5
alert( Math.min(1, 2) ); // 1
alert( Math.pow(2, 10) ); // 2의 10제곱 = 1024

// 비트 NOT 연산자를 사용한 기법
alert( ~2 ); // -3, -(2+1)과 같음
alert( ~1 ); // -2, -(1+1)과 같음
alert( ~0 ); // -1, -(0+1)과 같음
alert( ~-1 ); // 0, -(-1+1)과 같음
```


### 문자형

- character type 없음
- 큰따옴표: "Hello"
- 작은따옴표: 'Hello'
- 역 따옴표(백틱, backtick): `Hello`
- 문자열 길이: 문자열.length
- 문자 수정 X, 문자열로만 취급
- 모든 문자열은 UTF-16을 사용해 인코딩

```javascript
let str = "Hello";
let str2 = 'Single quotes are ok too';
let phrase = `can embed another ${str}`;

let name = "John";
// backtick 사용시 - 변수를 문자열 중간에 삽입하거나 계산 가능
alert( `Hello, ${name}!` ); // Hello, John!
alert( `the result is ${1 + 2}` ); // the result is 3

// multi-lines
let guestList = `손님:
 * John
 * Pete
 * Mary
`;
let guestList = "손님:\n * John\n * Pete\n * Mary";
alert( "\u00A9" ); // ©
alert( "\u{20331}" ); // 佫, 중국어(긴 유니코드)
alert( "\u{1F60D}" ); // 😍, 웃는 얼굴 기호(긴 유니코드)
alert( 'I\'m the Walrus!' ); // I'm the Walrus!

// 문자열 길이
alert( `My\n`.length ); // 3

// 문자 접근
let str = `Hello`;
alert( str[0] ); // H
alert( str.charAt(0) ); // H
alert( str[str.length - 1] ); // o; // 마지막 글자
// 대소문자
alert( 'Interface'.toUpperCase() ); // INTERFACE
alert( 'Interface'.toLowerCase() ); // interface

// 부분 문자열 찾기
let str = 'Widget with id';
alert( str.indexOf('Widget') ); // 0, str은 'Widget'으로 시작함
alert( str.indexOf('widget') ); // -1, indexOf는 대·소문자를 따지므로 원하는 문자열을 찾지 못함
alert( str.indexOf("id") ); // 1, "id"는 첫 번째 위치에서 발견됨 (Widget에서 id)

let str = "As sly as a fox, as strong as an ox";
let target = "as";
let pos = -1;
while ((pos = str.indexOf(target, pos + 1)) != -1) {
  alert( `위치: ${pos}` );
}

// bit not 연산자 사용 기법
let str = "Widget";
if (~str.indexOf("Widget")) {
  alert( '찾았다!' ); // 의도한 대로 동작합니다.
}

alert( "Widget with id".includes("Widget") ); // true
alert( "Widget".startsWith("Wid") ); // true, "Widget"은 "Wid"로 시작합니다.
alert( "Widget".endsWith("get") ); // true, "Widget"은 "get"으로 끝납니다.

// 부분 문자열 추출
let str = "stringify";
alert( str.slice(0, 5) ); // 'strin', 0번째부터 5번째 위치까지(5번째 위치의 글자는 포함하지 않음)
alert( str.slice(0, 1) ); // 's', 0번째부터 1번째 위치까지(1번째 위치의 자는 포함하지 않음)
alert( str.slice(2) ); // ringify, 2번째부터 끝까지
alert( str.slice(-4, -1) ); // gif

alert( str.substring(2, 6) ); // "ring"
alert( str.substring(6, 2) ); // "ring"

let str = "stringify";
alert( str.substr(2, 4) ); // ring, 두 번째부터 글자 네 개

// 문자열 비교 UTF-16
alert( 'a' > 'Z' ); // true
alert( 'Österreich' > 'Zealand' ); // true

alert( "z".codePointAt(0) ); // 122
alert( "Z".codePointAt(0) ); // 90
alert( String.fromCodePoint(90) ); // Z
alert( '\u005a' ); // Z

alert( 'Österreich'.localeCompare('Zealand') ); // -1
```

### `boolean`형

```javascript
let nameFieldChecked = true;
let ageFieldChecked = false;
let isGreater = 4 > 1;
```

### `null` 값

일반 null point의 이미가 아니라, `존재하지 않는(nothing)` 값, 
`비어 있는(empty)` 값, `알 수 없는(unknown)` 값을 나타내는 데 사용

```javascript
let age = null; // 나이(age)를 알 수 없거나 그 값이 비어있음
```

### `undefined` 값

`undefined`는 '값이 할당되지 않은 상태’를 나타낼 때 사용

```javascript
let age;
alert(age); // 'undefined' 출력
```

### `object`

- = `json`?
- This is non-primitive type.
- **property**: {key: value} pair, key=문자형, 심볼형, value=모든 자료형
- **dot notation**, **square bracket notation**으로 property 접근,추가,삭제 가능
- `in`을 사용해 property 존재여부 확인 가능 (`true`/`false`)
- property는 object에 참조로 구현됨, object를 다른 변수에 할당할 경우 같은 property를 참조함

```javascript
// empty object
let user = new Object(); // 객체 생성자로 생성
let user = {};  // 객체 리터럴(object literal)로 생성

let user = {     // 객체
  name: "John",  // 키: "name",  값: "John"
  age: 30,       // 키: "age", 값: 30
  "likes birds": true  // 복수의 단어는 따옴표
};

alert( user.name ); // John, dot notation을 사용해 접근
alert( user["name"] ); // square bracket notation을 사용한 접근

user.likes birds = false; // error, 구문해석 불능
user["likes birds"] = false; // ok

alart( user.weight === undefined ); // property가 없을 경우, undefined
alart( user["weight"] === undefined );
alart( "key" in user ); // in 연산자 사용해 존재유무 확인


user.isAdmin = true; // add property
delete user.age; // delete a property
delete user["likes birds"];

let user = {
  name: "John",
  age: 30,
};

// computed property!
let fruit = "apple"
let fruit2 = "banana"
let bag = {
  [fruit]: 5, // fruit를 key로 사용
  [fruit + 'Computers']: 5 // key로 복합 연산 가능
};
bag[fruit2] = 6; // fruit2를 key로 사용
alert( bag.apple ); // fruit에 "apple"이 할당되었다면, 5가 출력

// property value shorthand (단축 구문)
function makeUser(name, age) {
  return {
    name, // name: name 과 같음
    age,  // age: age 와 같음
    // ...
  };
}

let user = {
  name,  // name: name 과 같음
  age: 30
};

// property iteration
for (key in object) {
  // 각 프로퍼티 키(key)를 이용하여 본문(body)을 실행합니다.
}

// object의 property는 선언할 필요없음
function marry(man, woman) {
  woman.husband = man;
  man.wife = woman;

  return {
    father: man,
    mother: woman
  }
}

let family = marry({
  name: "John"
}, {
  name: "Ann"
});
```

#### const object의 내부 정보 변경 가능

```javascript
const user = {
  name: "John"
};
user.name = "Pete"; // (*)
alert(user.name); // Pete
```

#### 예약어 키로 사용 - ok

```javascript
// 예약어를 키로 사용해도 괜찮습니다.
let obj = {
  for: 1,
  let: 2,
  return: 3
};
```

####  키에 숫자 0을 넣으면 문자열 "0"으로 자동변환

```javascript
let obj = {
  0: "test" // "0": "test"와 동일합니다.
};

alert( obj["0"] ); // test
alert( obj[0] ); // test
```

#### object 비교

```javascript
let a = {};
let b = a; // 참조에 의한 복사
alert( a == b ); // true, 두 변수는 같은 객체를 참조합니다.
alert( a === b ); // true

let a = {};
let b = {}; // 독립된 두 객체
alert( a == b ); // false
```

#### 객체 복사, 병합과 Object.assign

- property loop를 통해서 복사
- `Object.assign(dest, [src1, src2, src3...])` 사용
- 깊은 복사(deep cloning)는 직접 만들던지, lodash의 `_.cloneDeep(obj)` 사용

```javascript
let user = {
  name: "John",
  age: 30
};

let clone = {}; // 새로운 빈 객체
// 빈 객체에 user 프로퍼티 전부를 복사해 넣습니다.
for (let key in user) {
  clone[key] = user[key];
}

// Object.assign 사용
let user = { name: "John" };
let permissions1 = { canView: true };
let permissions2 = { canEdit: true };

// permissions1과 permissions2의 프로퍼티를 user로 복사
Object.assign(user, permissions1, permissions2);
```

#### Methods and this

- 개체의 동작을 명세
- = property에 할당된 함수
- `this`: method에서 해당 객체의 property 접근을 위해 사용
- Arrow function은 `this`가 없음, 별개의 this가 만들어지는 건 원하지 않고, 외부 컨텍스트에 있는 this를 이용하고 싶은 경우 화살표 함수가 유용

```javascript
let user = {
  sayHi: function() {
    alert("Hello");
  }
};

// method 축약형
let user = {
  sayHi() { // "sayHi: function()"과 동일
    alert("Hello");
  }
};

// Arrow function은 상위 namespace의 this를 가져옴
let user = {
  firstName: "보라",
  sayHi() {
    let arrow = () => alert(this.firstName);
    arrow();
  }
};

user.sayHi(); // sayHi의 this 차용; 결과 '보라'
```

#### 객체 생성 함수 constructor function

- 생성자 함수(constructor function)와 일반 함수에 기술적인 차이는 없음
- 
- 관례1: 함수 이름의 첫 글자는 대문자로 시작
- 관례2: 반드시 'new' 연산자를 붙여 실행
- new Constructor(...) 시
  - 빈 객체 생성, this에 할당
  - 함수 실행 => property 추가
  - 생성된 this를 반환


```javascript
// constructor function
function User(name) {
  // this = {};  (빈 객체가 암시적으로 만들어짐)
  // 새로운 프로퍼티를 this에 추가함
  this.name = name;
  this.isAdmin = false;
  // return this;  (this가 암시적으로 반환됨)
}

let user = new User("보라");
alert(user.isAdmin); // false

// 익명 생성자 (anonymous constructor) - 한번만 사용
let user = new function() {
  this.name = "John";
  this.isAdmin = false;
};

// new.target으로 constructor가 new함께 호출되었는지 알 수 있음.
function User() {
  alert(new.target);
}
// 'new' 없이 호출함
User(); // undefined
// 'new'를 붙여 호출함
new User(); // function User { ... }

// 생성자 () 괄호 생략
let user = new User; // <-- 괄호가 없음
let user = new User(); // 위 코드와 똑같이 동작합니다.

// 생성자에서 method 추가
function User(name) {
  this.name = name;

  this.sayHi = function() {
    alert( "제 이름은 " + this.name + "입니다." );
  };
}
let bora = new User("이보라");
bora.sayHi(); // 제 이름은 이보라입니다.
```

#### 옵셔널 체이닝(optional chaining) `?.`, `?.()`, `?.[]`

- property 존재 유무 평가
- method에서도 사용 가능
- property 쓰기에는 사용 불가

```javascript
let user = {}; // 주소 정보가 없는 사용자
alert(user.address.street); // TypeError: Cannot read property 'street' of undefined
alert( user && user.address && user.address.street ); // undefined, 에러가 발생하지 않습니다.
alert( user?.address?.street ); // undefined, 에러가 발생하지 않습니다. <= optional chaining

// 함수 호출에도 사용
let user1 = {
  firstName: "Violet"
  admin() {
    alert("관리자 계정입니다.");
  }
}

let user2 = {};

user1.admin?.(); // 관리자 계정입니다.
user2.admin?.();
alert( user1?.["firstName"] ); // Violet
```

#### object 순회 함수

아래 함수는 ‘진짜’ 배열을 반환하며, Symbol은 무시

- `Object.keys(obj)`: 객체의 키만 담은 배열을 반환
- `Object.values(obj)`: 객체의 값만 담은 배열을 반환
- `Object.entries(obj)`: [키, 값] 쌍을 담은 배열을 반환
- `Object.fromEntries(array)`: `Object.entries(obj)`의 역변환

```javascript
let user = {
  name: "John",
  age: 30
};
// Object.keys(user) = ["name", "age"]
// Object.values(user) = ["John", 30]
// Object.entries(user) = [ ["name","John"], ["age",30] ]
for (let value of Object.values(user)) {
  alert(value);
}

let prices = {
  banana: 1,
  orange: 2,
  meat: 4,
};
let doublePrices = Object.fromEntries(
  // 객체를 배열로 변환해서 배열 전용 메서드인 map을 적용하고 fromEntries를 사용해 배열을 다시 객체로 되돌립니다.
  Object.entries(prices).map(([key, value]) => [key, value * 2])
);
alert(doublePrices.meat); // 8
```

### `symbol`

- 외부 객체에 자신의 정보를 hidden property로 추가시 유용
- `for..in` 반복문에서 배제
- `Object.assign`은 symbol도 복사함
- `global symbol registry`: global 영역에 symbol을 저장, script 곳곳에서 symbol을 호출하여 사용
- `system symbol`: javascript 내부 symbol; https://tc39.github.io/ecma262/#sec-well-known-symbols
  - `Symbol.hasInstance`
  - `Symbol.isConcatSpreadable`
  - `Symbol.iterator`
  - `Symbol.toPrimitive`

```javascript
let id = Symbol("id");
alert(id); // TypeError: Cannot convert a Symbol value to a string
alert(id.toString()); // Symbol(id)가 얼럿 창에 출력됨
alert(id.description); // id

// Hidden property
let user = { // 서드파티 코드에서 가져온 객체
  name: "John"
};
let id = Symbol("id");
user[id] = 1;
alert( user[id] ); // 심볼을 키로 사용해 데이터에 접근할 수 있습니다.

// 다른 방법
let id = Symbol("id");
let user = {
  name: "John",
  [id]: 123 // "id": 123은 안됨
};

// Object.assign과 symbol 동작
let id = Symbol("id");
let user = {
  [id]: 123
};

let clone = Object.assign({}, user);

alert( clone[id] ); // 123
```

```javascript
// 전역 레지스트리에서 심볼 읽기
let id = Symbol.for("id"); // 심볼이 존재하지 않으면 새로운 심볼 생성
let idAgain = Symbol.for("id");
alert( id === idAgain ); // true
// 심볼을 이용해 이름을 얻음
alert( Symbol.keyFor(id) ); // "id"
alert( Symbol.keyFor(localSymbol) ); // 전역 심볼이 아님, undefined 반환
```

#### 객체의 형변환; Symbol.toPrimitive

- 객체에 `obj[Symbol.toPrimitive](hint)`메서드가 있는지 찾고, 있다면 메서드를 호출합니다.
- `Symbol.toPrimitive`는 시스템 심볼로, 심볼형 키로 사용됩니다.
- 1에 해당하지 않고 hint가 "string"이라면,
- `obj.toString()`이나 `obj.valueOf()`를 호출합니다(존재하는 메서드만 실행됨).
- 1과 2에 해당하지 않고, hint가 "number"나 "default"라면
- `obj.valueOf()`나 `obj.toString()`을 호출합니다(존재하는 메서드만 실행됨).
- hint는 "string", "number", "default" 중 하나

```javascript
let user = {
  name: "John",
  money: 1000,

  [Symbol.toPrimitive](hint) {
    alert(`hint: ${hint}`);
    return hint == "string" ? `{name: "${this.name}"}` : this.money;
  }
};

// 데모:
alert(user); // hint: string -> {name: "John"}
alert(+user); // hint: number -> 1000
alert(user + 500); // hint: default -> 1500

// toString, valueOf
let user = {
  name: "John",
  money: 1000,

  // hint가 "string"인 경우
  toString() {
    return `{name: "${this.name}"}`;
  },

  // hint가 "number"나 "default"인 경우
  valueOf() {
    return this.money;
  }

};

alert(user); // toString -> {name: "John"}
alert(+user); // valueOf -> 1000
alert(user + 500); // valueOf -> 1500
```

#### iterable 객체 만들기; Symbol.iterator

- 주로 array에 사용되던 `for..in` 이 동작 가능함
- `next()` 다음 값 iterater를 반환

```javascript
let range = {
  from: 1,
  to: 5,

  [Symbol.iterator]() {
    this.current = this.from;
    return this;
  },

  next() {
    if (this.current <= this.to) {
      return { done: false, value: this.current++ };
    } else {
      return { done: true };
    }
  }
};

for (let num of range) {
  alert(num); // 1, then 2, 3, 4, 5
}
```

문자열 = iterable

```javascript
for (let char of "test") {
  // 글자 하나당 한 번 실행됩니다(4회 호출).
  alert( char ); // t, e, s, t가 차례대로 출력됨
}
```

`Array.from`으로 객체를 배열로 만들기

```javascript
let arrayLike = {
  0: "Hello",
  1: "World",
  length: 2
};

let arr = Array.from(arrayLike); // (*)
alert(arr.pop()); // World (메서드가 제대로 동작합니다.)
```

### `Array`

- push와 pop은 빠르지만 shift와 unshift는 느림
- https://ko.javascript.info/array
- for..in loop 사용시 내부 property로 같이 순회하므로 사용 X

```javascript
// 선언
let arr = new Array();
let arr = [];
let arr = new Array(2); // [undefined, undefined]
let fruits = ["사과", "오렌지", "자두"];
let fruits = [
  "사과",
  "오렌지",
  "자두",
];
let matrix = [
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
];

// 접근
alert( fruits[0] ); // 사과
fruits[2] = '배';

// 크기
alert( fruits.length ); // 3

// 복합 배열
let arr = [ '사과', { name: '이보라' }, true, function() { alert('안녕하세요.'); } ];

// pop·push와 shift·unshift
let fruits = ["사과", "오렌지", "배"];
alert( fruits.pop() ); // 배열에서 "배"를 제거하고 제거된 요소를 얼럿창에 띄웁니다.
alert( fruits ); // 사과,오렌지
fruits.push("배");
alert( fruits ); // 사과,오렌지,배
alert( fruits.shift() ); // 배열에서 "사과"를 제거하고 제거된 요소를 얼럿창에 띄웁니다.
alert( fruits ); // 오렌지,배
fruits.unshift('사과'); // 앞에 요소 추가
alert( fruits ); // 사과,오렌지,배

// 여러 요소 삽입
fruits.push("오렌지", "배");
fruits.unshift("파인애플", "레몬");

// 요소 삭제
let arr = ["I", "go", "home"];
delete arr[1]; // "go"를 삭제합니다.
arr.splice(1, 1); // 인덱스 1부터 요소 한 개를 제거

// 요소 대체
let arr = ["I", "study", "JavaScript", "right", "now"];
// 처음(0) 세 개(3)의 요소를 지우고, 이 자리를 다른 요소로 대체합니다.
arr.splice(0, 3, "Let's", "dance");
alert( arr ) // now ["Let's", "dance", "right", "now"]

// 요소 추가 (deleteCount를 0으로 설정하면 요소를 제거하지 않으면서 새로운 요소를 추가)
arr.splice(2, 0, "complex", "language");

// slicing, concat
let arr = [1, 2];
// arr의 요소 모두와 [3,4]의 요소 모두를 한데 모은 새로운 배열이 만들어집니다.
alert( arr.concat([3, 4]) ); // 1,2,3,4
// arr의 요소 모두와 [3,4]의 요소 모두, [5,6]의 요소 모두를 모은 새로운 배열이 만들어집니다.
alert( arr.concat([3, 4], [5, 6]) ); // 1,2,3,4,5,6
// arr의 요소 모두와 [3,4]의 요소 모두, 5와 6을 한데 모은 새로운 배열이 만들어집니다.
alert( arr.concat([3, 4], 5, 6) ); // 1,2,3,4,5,6

let arr = ["t", "e", "s", "t"];
alert( arr.slice(1, 3) ); // e,s (인덱스가 1인 요소부터 인덱스가 3인 요소까지를 복사(인덱스가 3인 요소는 제외))
alert( arr.slice(-2) ); // s,t (인덱스가 -2인 요소부터 제일 끝 요소까지를 복사)

// 반복문, 순회
let arr = ["사과", "오렌지", "배"];

for (let i = 0; i < arr.length; i++) {
  alert( arr[i] );
}

// for..of
for (let fruit of fruits) {
  alert( fruit );
}

arr.forEach(function(item, index, array) {
  // 요소에 무언가를 할 수 있습니다.
});

["Bilbo", "Gandalf", "Nazgul"].forEach(alert);
["Bilbo", "Gandalf", "Nazgul"].forEach((item, index, array) => {
  alert(`${item} is at index ${index} in ${array}`);
});

// 배열 탐색: indexOf, lastIndexOf와 includes
let arr = [1, 0, false];
alert( arr.indexOf(0) ); // 1
alert( arr.indexOf(false) ); // 2
alert( arr.indexOf(null) ); // -1
alert( arr.includes(1) ); // true

const arr = [NaN];
alert( arr.indexOf(NaN) ); // -1 (완전 항등 비교 === 는 NaN엔 동작하지 않으므로 0이 출력되지 않습니다.)
alert( arr.includes(NaN) );// true (NaN의 여부를 확인하였습니다.)

let result = arr.find(function(item, index, array) {
});
let users = [
  {id: 1, name: "John"},
  {id: 2, name: "Pete"},
  {id: 3, name: "Mary"}
];
let user = users.find(item => item.id == 1);
alert(user.name); // John

// 조건 탐색
let users = [
  {id: 1, name: "John"},
  {id: 2, name: "Pete"},
  {id: 3, name: "Mary"}
];

// 앞쪽 사용자 두 명을 반환합니다.
let someUsers = users.filter(item => item.id < 3);
alert(someUsers.length); // 2
```

#### mapping function for array

배열을 변형시키거나 요소를 재 정렬해주는 메서드

```javascript
let result = arr.map(function(item, index, array) {
  // 요소 대신 새로운 값을 반환합니다.
});

let lengths = ["Bilbo", "Gandalf", "Nazgul"].map(item => item.length);
alert(lengths); // 5,7,6
```

#### sort

```javascript
let arr = [ 1, 2, 15 ];
arr.sort(); // arr 내부가 재 정렬됩니다.
alert( arr );  // 1, 15, 2

function compareNumeric(a, b) {
  if (a > b) return 1;
  if (a == b) return 0;
  if (a < b) return -1;
}
let arr = [ 1, 2, 15 ];
arr.sort(compareNumeric);
alert(arr);  // 1, 2, 15

[1, -2, 15, 2, 0, 8].sort(function(a, b) {
  alert( a + " <> " + b );
  return a - b;
});

// arrow function 사용
arr.sort( (a, b) => a - b );

// 문자열 정렬시에는 localeCompare 사용
let countries = ['Österreich', 'Andorra', 'Vietnam'];
alert( countries.sort( (a, b) => a > b ? 1 : -1) ); // Andorra, Vietnam, Österreich (제대로 정렬이 되지 않았습니다.)
alert( countries.sort( (a, b) => a.localeCompare(b) ) ); // Andorra,Österreich,Vietnam (제대로 정렬되었네요!)

// 역정렬
let arr = [1, 2, 3, 4, 5];
arr.reverse();
alert( arr ); // 5,4,3,2,1
```

#### 문자열 <==> 배열

```javascript
let names = 'Bilbo, Gandalf, Nazgul';
let arr = names.split(', ');
for (let name of arr) {
  alert( `${name}에게 보내는 메시지` ); // Bilbo에게 보내는 메시지
}

let str = "test";
alert( str.split('') ); // t,e,s,t


let arr = ['Bilbo', 'Gandalf', 'Nazgul'];
let str = arr.join(';'); // 배열 요소 모두를 ;를 사용해 하나의 문자열로 합칩니다.
alert( str ); // Bilbo;Gandalf;Nazgul
```

#### reduce와 reduceRight (누산기)

```javascript
let value = arr.reduce(function(accumulator, item, index, array) {
  // ...
}, [initial]);
let arr = [1, 2, 3, 4, 5];
let result = arr.reduce((sum, current) => sum + current, 0);
alert(result); // 15
```

#### 배열인지 확인

```javascript
alert(Array.isArray({})); // false
alert(Array.isArray([])); // true
```

### `map`

객체와 유사하지만, key에 다양한 자료형 지원

- 되도록 get(), set()을 사용?
- map은 키로 객체를 허용
- `SameValueZero`라 불리는 알고리즘을 사용해 값의 등가 여부를 확인
- call chaining 사용가능: 호출마다 map 자신을 반환함
- 삽입 순서를 기억함 = ordered map
- `Object.entries`: 객체를 map으로 바꾸기
- `Object.fromEntries`: map을 객체로 바꾸기

#### 제공 함수

- `new Map()`: 생성
- `map.set(key, value)`: {key: value} 저장
- `map.get(key)`: value 반환, key가 존재하지 않으면 undefined
- `map.has(key)`: true if the key exists
- `map.delete(key)`: key, value 삭제
- `map.clear()`: clear all in the map.
- `map.size`:  size of the map
- `map.keys()`: 각 요소의 키를 모은 반복 가능한(iterable, 이터러블) 객체를 반환; `for..of`
- `map.values()`: 각 요소의 값을 모은 이터러블 객체를 반환; `for..of`
- `map.entries()`: 요소의 [키, 값]을 한 쌍으로 하는 이터러블 객체를 반환; `for..of`
- `map.forEach((value, key, map) => {})`: map의 {key: value} 순회

```javascript
let map = new Map();

map.set('1', 'str1');   // 문자형 키
map.set(1, 'num1');     // 숫자형 키
map.set(true, 'bool1'); // 불린형 키

// map은 key의 타입을 변환시키지 않고 그대로 유지
alert( map.get(1)   ); // 'num1'
alert( map.get('1') ); // 'str1'
alert( map.size ); // 3

// map call chaining
map.set('1', 'str1')
  .set(1, 'num1')
  .set(true, 'bool1');

// map 요소 반복문
let recipeMap = new Map([
  ['cucumber', 500],
  ['tomatoes', 350],
  ['onion',    50]
]);

// 키(vegetable)를 대상으로 순회합니다.
for (let vegetable of recipeMap.keys()) {
  alert(vegetable); // cucumber, tomatoes, onion
}

// 값(amount)을 대상으로 순회합니다.
for (let amount of recipeMap.values()) {
  alert(amount); // 500, 350, 50
}

// [키, 값] 쌍을 대상으로 순회합니다.
for (let entry of recipeMap) { // recipeMap.entries()와 동일합니다.
  alert(entry); // cucumber,500 ...
}

// 각 (키, 값) 쌍을 대상으로 함수를 실행
recipeMap.forEach( (value, key, map) => {
  alert(`${key}: ${value}`); // cucumber: 500 ...
});

// map으로 변환
let map = new Map([ // 각 요소가 [키, 값] 쌍인 배열
  ['1',  'str1'],
  [1,    'num1'],
  [true, 'bool1']
]);
alert( map.get('1') ); // str1

let obj = {
  name: "John",
  age: 30
};

let map = new Map(Object.entries(obj));
alert( map.get('name') ); // John

// map을 object로 변환
let prices = Object.fromEntries([
  ['banana', 1],
  ['orange', 2],
  ['meat', 4]
]);
```

### `set`

값의 중복 불가한 collection으로 call chaining 사용가능

- `new Set(iterable)`: set 생성; iterable (보통 array) 일 경우 값 복사
- `set.add(value)`: 값을 추가
- `set.delete(value)`: 값 삭제, it returns true or false.
- `set.has(value)`: 값 존재 유무 반환
- `set.clear()`: set의 모든 값 삭제
- `set.size`: set 내 요소의 수
- `set.forEach((value, valueAgain, set) => {})`: set 요소 반복 순회
- `set.keys()`
- `set.values()`
- `set.entries()`

```javascript
let set = new Set();
let john = { name: "John" };
let pete = { name: "Pete" };
let mary = { name: "Mary" };
set.add(john);
set.add(pete);
set.add(mary);
set.add(john);
set.add(mary);
alert( set.size ); // 3
for (let user of set) {
  alert(user.name); // // John, Pete, Mary 순으로 출력됩니다.
}
// forEach
set.forEach((value, valueAgain, set) => {
  alert(value);
});
```


### `WeakMap` and `WeakSet`

WeakMap은 object만을 key로 사용하며, object가 unreachable하면, WeakMap에 key도 Garbage collection에 의해 메모리에서 자동해제됨

> WeakMap은 부차적인 데이터를 저장할 곳이 필요할 때 유용 e.g. 사용자 방문횟수, caching

- `weakMap.get(key)`
- `weakMap.set(key, value)`
- `weakMap.delete(key)`
- `weakMap.has(key)`

WeakSet도 WeakMap과 유사하나 동작을 수행한다.

- `new WeakSet(iterable)`: set 생성; iterable (보통 array) 일 경우 값 복사
- `weakSet.add(value)`: 값을 추가
- `weakSet.delete(value)`: 값 삭제, it returns true or false.
- `weakSet.has(value)`: 값 존재 유무 반환

```javascript
let john = { name: "John" };
let weakMap = new WeakMap();
weakMap.set(john, "...");
john = null; // 참조를 덮어씀
// john을 나타내는 객체는 이제 메모리에서 지워집니다!

// caching
let cache = new WeakMap();
// 연산을 수행하고 그 결과를 위크맵에 저장합니다.
function process(obj) {
  if (!cache.has(obj)) {
    let result = /* 연산 수행 */ obj;
    cache.set(obj, result);
  }
  return cache.get(obj);
}

// 📁 main.js
let obj = {/* ... 객체 ... */};
let result1 = process(obj);
let result2 = process(obj);

// 객체가 쓸모없어지면 아래와 같이 null로 덮어씁니다.
obj = null;
```

### `Date`

UTC 기준(UTC+0) 1970년 1월 1일 0시 0분 0초에서 milliseconds 후의 시간값을 저장하는 object

- `new Date(milliseconds)`
- `new Date(datestring)`
- `new Date(year, month, date, hours, minutes, seconds, ms)`
- `getFullYear()`, `getMonth()`, `getDate()`, `getHours()`, `getMinutes()`, `getSeconds()`, `getMilliseconds()`
- `getDay()`: 요일
- `getUTCFullYear()`, `getUTCMonth()`, `getUTCDay()`: UTC timezone
- `getTime()`: returns timestamp
- `getTimezoneOffset()`: offset from UTC basetime
- `setFullYear(year, [month], [date])`
- `setMonth(month, [date])`
- `setDate(date)`
- `setHours(hour, [min], [sec], [ms])`
- `setMinutes(min, [sec], [ms])`
- `setSeconds(sec, [ms])`
- `setMilliseconds(ms)`
- `setTime(milliseconds)`

```javascript
let date = new Date("2017-01-26");
let now = new Date();
alert( now ); // 현재 날짜 및 시간이 출력됨
let timestamp = Date.now() // current timestamp
```

#### Autocorrection

```javascript
let date = new Date(2013, 0, 32); // 2013년 1월 32일
alert(date); // 2013년 2월 1일
alert(+date); // 숫자형으로 변환 (timestamp)
```

#### Benchmarking Test

```javascript
function diffGetTime(date1, date2) {
  return date2.getTime() - date1.getTime();
}
```

#### Date.parse

`YYYY-MM-DDTHH:mm:ss.sssZ+-hh:mm` 형식의 문자를 parsing하여 Data object로 반환

```javascript
let ms = Date.parse('2012-01-26T13:51:50.417-07:00');
```

## JSON (JavaScript Object Notation) Serialization

- javascript의 object의 serialization 기술로 RFC4627로 표준화
- `JSON.stringify`: It serializes an object to a JSON-encoded string.
- `JSON.parse`: It de-serializes an object from a JSON-encoded string.
- 표현 자료형: `object`, `array`, `string`, `number`, `boolean`, `null`
- 이외 자료형은 모두 ignored
- 순환 참조 object가 있을 경우 error

```javascript
let student = {
  name: 'John',
  age: 30,
  isAdmin: false,
  courses: ['html', 'css', 'js'],
  wife: null
};
let json = JSON.stringify(student);
alert(typeof json); // string
alert(json); // {"name":"John","age":30,"isAdmin":false,"courses":["html","css","js"],"wife":null}
alart(JSON.stringify(json));

// 순환 참조시 error
let room = {
  number: 23
};
let meetup = {
  title: "Conference",
  participants: ["john", "ann"]
};
meetup.place = room;       // meetup은 room을 참조합니다.
room.occupiedBy = meetup; // room은 meetup을 참조합니다.
JSON.stringify(meetup); // Error: Converting circular structure to JSON

// 원하는 값만 추출
// let json = JSON.stringify(value[, replacer, space])
let room = {
  number: 23
};
let meetup = {
  title: "Conference",
  participants: [{name: "John"}, {name: "Alice"}],
  place: room // meetup references room
};
room.occupiedBy = meetup; // room references meetup
alert( JSON.stringify(meetup, ['title', 'participants', 'place', 'name', 'number']) ); // {"title":"Conference","participants":[{"name":"John"},{"name":"Alice"}],"place":{"number":23}}

// replacer 사용
alert( JSON.stringify(meetup, function replacer(key, value) {
  return (key == 'occupiedBy') ? undefined : value;
})); // {"title":"Conference","participants":[{"name":"John"},{"name":"Alice"}],"place":{"number":23}}

// reviver 사용
let str = '{"title":"Conference","date":"2017-11-30T12:00:00.000Z"}';
let meetup = JSON.parse(str);
alert( meetup.date.getDate() ); // 에러!
meetup = JSON.parse(str, function(key, value) {
  if (key == 'date') return new Date(value);
  return value;
});
alert( meetup.date.getDate() ); // 30
```

### `toJSON()` for customized serialization

```javascript
let room = {
  number: 23,
  toJSON() {
    return this.number;
  }
};
let meetup = {
  title: "Conference",
  room
};
alert( JSON.stringify(room) ); // 23
alert( JSON.stringify(meetup) ); // {"title":"Conference","room":23}
```

## `typeof`

- `typeof X` 연산자는 인수의 자료형을 문자열로 반환
- 자료형에 따라 처리 방식을 다르게 할 경우 사용
- 변수의 자료형 확인에 사용
- 연산자형 사용: `typeof x`
- 함수형 사용: `typeof(x)`

```javascript
typeof undefined // "undefined"
typeof 0 // "number"
typeof 10n // "bigint"
typeof true // "boolean"
typeof "foo" // "string"
typeof Symbol("id") // "symbol"
typeof Math // "object"  (1)
typeof null // "object"  (2)
typeof alert // "function"  (3)
```

## Browser functions

- `alart`: 메시지가 있는 작은 창, 모달 창(modal window)을 띄움
- `prompt`: 메시지와 입력 필드(input field), 확인(OK) 및 취소(Cancel) 버튼이 있는 모달 창을 띄움
- `confirm`: 질문과 확인 및 취소 버튼이 있는 모달 창을 띄움

```javascript
let age = prompt('나이를 입력해주세요.', 100);
alert(`당신의 나이는 ${age}살 입니다.`);

let isBoss = confirm("당신이 주인인가요?");
alert( isBoss );
```

## 형변환 (type conversion)

```javascript
// 문자열 변환
let value = true; // boolean형
value = String(value); // 변수 value엔 문자열 "true"가 저장
alert(typeof value); // string

// 숫자로 변환
let str = "123";
let num = Number(str); // 문자열 "123"이 숫자 123으로 명시적 변환
alert(typeof num); // number
alert( "6" / "2" ); // 3, 문자열이 숫자형으로 자동변환된 후 연산이 수행

let age = Number("임의의 문자열 123");
alert(age); // NaN, 형 변환 실패
alert( Number("   123   ") ); // 123
alert( Number("123z") );      // NaN ("z"를 숫자로 변환하는 데 실패함)
alert( Number(true) );        // 1
alert( Number(false) );       // 0

// boolean으로 변환
alert( Boolean(1) ); // 숫자 1(true)
alert( Boolean(0) ); // 숫자 0(false)
alert( Boolean("hello") ); // 문자열(true)
alert( Boolean("") ); // 빈 문자열(false)
alert( Boolean(NaN) ); // false
alert( Boolean(undefined) ); // false
alert( Boolean(null) ); // false
```

## 연산자

- `+`: 덧셈 연산자
- `-`: 뺄셈 연산자
- `*`: 곱셈 연산자
- `/`: 나눗셈 연산자
- `%`: 나머지 연산자
- `**`: 거듭제곱 연산자
- `&`: AND 비트 연산자
- `|` OR 비트 연산자
- `^` XOR 비트 연산자
- `~` NOT 비트 연산자
- `<<`: LEFT SHIFT
- `>>`: RIGHT SHIFT
- `>>>`: ZERO-FILL RIGHT SHIFT
- `||`: OR 논리연산자
- `&&`: AND 논리연산자
- `!`: NOT 논리연산자

```javascript
// 숫자
alert( 5 % 2 ); // 5를 2로 나눈 후의 나머지인 1을 출력
alert( 8 % 3 ); // 8을 3으로 나눈 후의 나머지인 2를 출력
alert( 2 ** 4 ); // 16 (2 * 2 * 2 * 2)
alert( 4 ** (1/2) ); // 2 (1/2 거듭제곱은 제곱근)
alert( 8 ** (1/3) ); // 2 (1/3 거듭제곱은 세제곱근)

// 문자열
let s = "my" + "string";
alert(s); // mystring
alert( 2 + '1' ); // "21"
alert(2 + 2 + '1' ); // '221'이 아니라 '41'이 출력됩니다.
alert( 6 - '2' ); // 4, '2'를 숫자로 바꾼 후 연산이 진행됩니다.
alert( '6' / '2' ); // 3, 두 피연산자가 숫자로 바뀐 후 연산이 진행됩니다.
// 숫자형이 아닌 피연산자는 숫자형으로 변화합니다.
alert( +true ); // 1
alert( +"" );   // 0

let apples = "2";
let oranges = "3";
alert( apples + oranges ); // 23, 문자열 + 연산
alert( +apples + +oranges ); // 5 숫자변환 -> 숫자 + 연산
```

### 연산자 우선순위 Operator precedence table

https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence

### 할당 연산자

```javascript
let a = 1;
let b = 2;
let c = 3 - (a = b + 1); // a = 3, c = 0

alert( a ); // 3
alert( c ); // 0

a = b = c = 2 + 2;

alert( a ); // 4
alert( b ); // 4
alert( c ); // 4

let n = 2;
n += 5; // n = n + 5
n *= 2; // n = n * 2
alert( n ); // 14
n *= 3 + 5; // n *= 8

let counter = 2;
counter++; // counter = counter + 1
counter--; // counter = counter - 1

counter = 1;
let a = ++counter;
alert(a); // 2

counter = 1;
a = counter++;
alert(a); // 1

```

### 쉼표 연산자

```javascript
// 한 줄에서 세 개의 연산이 수행됨
for (a = 1, b = 3, c = a * b; a < 10; a++) {
 ...
}
```

## 비교 연산자

- a `>` b
- a `<` b
- a `>=` b
- a `<=` b
- a `==` b
- a `!=` b

### error 비교

```javascript
const err = new Error('💣️ Something went wrong');
console.log(err instanceof Error); // 👉️ true
```

### 문자열 비교

자릿수 별로 사전순으로 비교; ascii등의 문자 순위

``` javascript
alert( 'Z' > 'A' ); // true
alert( 'Glow' > 'Glee' ); // true
alert( 'Bee' > 'Be' ); // true
```

### 다른형간의 비교

쓰지 않는게 좋을 듯 ..., 명시적으로 변형해 사용하시라.

### 일치 연산자(strict equality operator) ===

자료형의 동등 여부까지 검사; 피연산자 a와 b의 형이 다를 경우 a === b는 즉시 false를 반환함

### null이나 undefined와 비교하기

```javascript
alert( null === undefined ); // false
alert( null == undefined ); // true

alert( undefined > 0 ); // false (1)
alert( undefined < 0 ); // false (2)
alert( undefined == 0 ); // false (3)

alert( null > 0 );  // (1) false
alert( null == 0 ); // (2) false
alert( null >= 0 ); // (3) true

```

### 단락 평가 - short circuit evaluation

`bash` 동작과 동일

```javascript
true || alert("not printed");
false || alert("printed");
```

###  nullish 병합 연산자 (nullish coalescing operator) `??`

`a ?? b`의 평가 결과는 다음과 같습니다.

a가 `null`도 아니고 `undefined`도 아니면 `a`
그 외의 경우는 `b`

```javascript
x = a ?? b // nullish 아래와 동일하게 풀어쓸 수 있음.
x = (a !== null && a !== undefined) ? a : b;

// 사용예
let firstName = null;
let lastName = null;
let nickName = "바이올렛";
alert(firstName ?? lastName ?? nickName ?? "익명의 사용자"); // 바이올렛

// '??'와 '||'의 차이
let height = 0;
alert(height || 100); // 100
alert(height ?? 100); // 0
```


## 조건문

- `if`, `else if` and `else`
- `?`

```javascript
let year = prompt('ECMAScript-2015 명세는 몇 년도에 출판되었을까요?', '');
if (year == 2015) {
  alert( "정답입니다!" );
  alert( "아주 똑똑하시네요!" );
} else if (year > 2015) {
  alert( '숫자를 좀 더 내려보세요.' );
} else {
  alert( '오답입니다!' );
}

// let result = condition ? value1 : value2;
let accessAllowed = (age > 18) ? true : false;
let message = (age < 3) ? '아기야 안녕?' :
  (age < 18) ? '안녕!' :
  (age < 100) ? '환영합니다!' :
  '나이가 아주 많으시거나, 나이가 아닌 값을 입력 하셨군요!';

```

## 반복문

```javascript
// while
let i = 0;
while (i < 3) { // 0, 1, 2가 출력됩니다.
  alert( i );
  i++;
}

// do while
let i = 0;
do {
  alert( i );
  i++;
} while (i < 3);

// for
for (let i = 0; i < 3; i++) { // 0, 1, 2가 출력됩니다.
  alert(i);
}

// break
let sum = 0;
while (true) {
  let value = +prompt("숫자를 입력하세요.", '');
  if (!value) break; // (*)
  sum += value;
}
alert( '합계: ' + sum );

// object property 순회
for (key in object) {
  // 각 프로퍼티 키(key)를 이용하여 본문(body)을 실행합니다.
}

// for..of // array 순회
for (let fruit of fruits) {
}
```

## switch/case문

switch/case문의 인수엔 어떤 표현식이든 올 수 있음

```javascript
let a = 2 + 2;
let b = 0;
switch (a) {
  case b + 1:
    break;
  case 3:
    alert( '비교하려는 값보다 작습니다.' );
    break;
  case 4:
    alert( '비교하려는 값과 일치합니다.' );
    break;
  case 5:
  case 6:
    alert( '비교하려는 값보다 큽니다.' );
    break;
  default:
    alert( "어떤 값인지 파악이 되지 않습니다." );
}
```

## Function

- local variable: 함수 내 선언
- 매개변수에 값을 전달하지 않으면 그 값은 `undefined`
- 자바스크립트는 함수를 특별한 종류의 variable로 취급
- nested function 가능

```javascript
// Function declaration
let userName = 'John'; // global variable
function showMessage() {
  let message = 'Hello, ' + userName; // local variable
  alert(message);
}

showMessage(); // Hello, John

// Function arguments
function showMessage(from, text) { // 인수: from, text
  alert(from + ': ' + text);
}
showMessage(from, "Hello"); // *Ann*: Hello
showMessage("Ann"); // Ann: undefined

// Function arguments with default value
function showMessage(from, text= "no text given") { // 인수: from, text
  alert(from + ': ' + text);
}
showMessage("Ann"); // Ann: undefined

// Function return; return이 없을 경우 undefined
function sum(a, b) {
  return a + b;
}

// Function return 주의 사항
function sum(a, b) {
  return 
    a + b; // return후 ; 자동 삽입되므로 return시 개행 하지 않기
}
```

### Function Expression (함수 표현식)

```javascript
let sayHi = function() {
  alert( "Hello" );
}; // 함수 표현식에서 세미콜론 필수

alert( sayHi ); // 함수 코드 출력
alert( sayHi() ); // 함수 실행!!

```

### Arrow function (화살표 함수)

- 함수 축약 (python lambda)
- this를 가지지 않음 (선언된 상위 object의 this 가져다 씀)
- arguments 없음
- new와 함께 호출 불가능
- super도 없음
- 자체 '컨텍스트’가 없는 짧은 코드용

```javascript
let func = (arg1, arg2, ...argN) => expression

// e.g.
let sum = (a, b) => a + b;
let double = n => n * 2; // let double = function(n) { return n * 2 }
let sayHi = () => alert("안녕하세요!");

let age = prompt("나이를 알려주세요.", 18);
let welcome = (age < 18) ?
  () => alert('안녕') :
  () => alert("안녕하세요!");
welcome();

// example 1
let group = {
  title: "1모둠",
  students: ["보라", "호진", "지민"],

  showList() {
    this.students.forEach(
      student => alert(this.title + ': ' + student)
    );
  }
};

group.showList();

// example 2
function defer(f, ms) {
  return function() {
    setTimeout(() => f.apply(this, arguments), ms)
  };
}

function sayHi(who) {
  alert('안녕, ' + who);
}

let sayHiDeferred = defer(sayHi, 2000);
sayHiDeferred("철수"); // 2초 후 "안녕, 철수"가 출력됩니다.
```

### debugger

코드에 `debugger` 삽입시 breakpoint와 동일한 동작

```javascript
function hello(name) {
  let phrase = `Hello, ${name}!`;
  debugger;  // <-- 여기서 실행이 멈춥니다.
  say(phrase);
}
```

### console.log

```javascript
// 콘솔창을 열어 결과를 확인해 보세요.
for (let i = 0; i < 5; i++) {
  console.log("숫자", i);
}
```

### Function property

- `name`
- `length`: function parameter 갯수
- function property 추가 가능함

```javascript
function sayHi() {
  alert("Hi");
}
alert(sayHi.name); // sayHi

function f(sayHi = function() {}) {
  alert(sayHi.name); // sayHi (이름이 있네요!)
}
f();

// property 추가
function sayHi() {
  alert("Hi");
  sayHi.counter++;
}
sayHi.counter = 0; // 초깃값
sayHi(); // Hi
sayHi(); // Hi
alert( `호출 횟수: ${sayHi.counter}회` ); // 호출 횟수: 2회
```

### variable arguments

`...` 를 사용하여 가변 인자를 지원하는 함수를 만듬

```javascript
function ask(question, ...handlers) {
  let isYes = confirm(question);
  for(let handler of handlers) {
    if (handler.length == 0) {
      if (isYes) handler();
    } else {
      handler(isYes);
    }
  }
}
```

### Input argument spreading

`...`으로 array argument를 풀어 입력할 수 있음

```javascript
let arr1 = [1, -2, 3, 4];
let arr2 = [8, 3, -8, 1];
alert( Math.max(1, ...arr1, 2, ...arr2, 25) ); // 25
```

이러한 spreading 기법은 array 선언에서도 사용가능

```javascript
let arr = [3, 5, 1];
let arr2 = [8, 9, 15];
let merged = [0, ...arr, 2, ...arr2];
alert(merged); // 0,3,5,1,2,8,9,15 (0, arr, 2, arr2 순서로 합쳐집니다.)
```

### Closure and Lexical scoping 클로저와 어휘 범위 지정

- https://developer.mozilla.org/ko/docs/Web/JavaScript/Closures

javascript 코드 블럭 내에서 변수와 함수의 유효범위는 다음과 같은 특성을 가짐

- javascript에서 함수는 코드 블록을 가지며, 동작을 수행하는 특수한 변수
- 함수, 코드블록, 객체, 스크립트들은 각자 Lexical Environment라는 내부 객체를 가짐
- javascript는 선언된 지역 변수를 이 Lexical Env에 property로 저장

```javascript
function init() {
  var name = "Mozilla"; // name은 init에 의해 생성된 지역 변수이다.
  function displayName() { // displayName() 은 내부 함수이며, 클로저다.
    alert(name); // 부모 함수에서 선언된 변수를 사용한다.
  }
  displayName();
}
init();
```

- 각 Lexical Env는 외부 (상위) Lexical Env에 대한 reference를 가짐 (e.g. 위 `displayName()`의 외부 Lexical Env는 `init()`)
- 위에서 displayName()과 같이 리턴되는 함수를 `closure`라 하며, 
- `closure`는 모함수 init()의 Lexical Env를 참조할 수 있음.
- `closure`를 사용해 javascript에서는 private method를 구현함

아래 코드는 프라이빗 함수와 변수에 접근하는 퍼블릭 함수를 정의하기 위해 클로저를 사용하는 방법을 보여준다. 이렇게 클로저를 사용하는 것을 **모듈 패턴**이라 한다.

```javascript
var counter = (function() {
  var privateCounter = 0;
  function changeBy(val) {
    privateCounter += val;
  }
  return {
    increment: function() {
      changeBy(1);
    },
    decrement: function() {
      changeBy(-1);
    },
    value: function() {
      return privateCounter;
    }
  };
})();

console.log(counter.value()); // logs 0
counter.increment();
counter.increment();
console.log(counter.value()); // logs 2
counter.decrement();
console.log(counter.value()); // logs 1
```

### new Function

- `new`를 사용한 함수 생성
- 다른 언어에서는 string으로 표현된 자신의 코드 실행하는 expr
- 사용예) 외부에서 수신한 string function의 실행

```javascript
let sum = new Function('a', 'b', 'return a + b');
alert( sum(1, 2) ); // 3

let sayHi = new Function('alert("Hello")');
sayHi(); // Hello
```

## Time scheduling - `setTimeout`, `setInterval`

```javascript
// 설정
let timerId = setTimeout(func|code, [delay], [arg1], [arg2], ...)
let timerId = setInterval(func|code, [interval], [arg1], [arg2], ...)

// 취소
clearTimeout(timerId);
```

`setTimeout`의 delay가 0일 경우, 최대한 빨리 ..


## Testing - Mocha

BDD (Behavior Driven Development)는 테스트(test), 문서(documentation), 예시(example)를 한데 모아놓은 개념

```javascript
function pow(x, n) {
    if (n < 0) return NaN;
    if (Math.round(n) != n) return NaN;
    
    let p = 1
    for (let i = 0; i < n; i++) {
        p = x * p ;
    }
    return p;
}

// ...

describe("pow", function() {
  it("주어진 숫자의 n 제곱", function() {
    assert.equal(pow(2, 3), 8);
  });
});

// nested testing
describe("pow.2", function () {
  describe("case 1", function () {
    before(() => console.log("testing starts"));
    after(() => console.log("testing ends"));

    beforeEach(() => console.log("each starts"));
    afterEach(() => console.log("each ends"));

    it("10^10", function () {
        assert.equal(pow(10, 10), 10000000000);
    });
    it("10^5", function () {
        assert.equal(pow(10, 6), 1000000);
    });
  })
})
```

### chai (assertion logic) 

- assert.equal(value1, value2) – value1과 value2의 동등성을 확인합니다(value1 == value2).
- assert.strictEqual(value1, value2) – value1과 value2의 일치성을 확인합니다(value1 === value2).
- assert.notEqual, assert.notStrictEqual – 비 동등성, 비 일치성을 확인합니다.
- assert.isTrue(value) – value가 true인지 확인합니다(value === true).
- assert.isFalse(value) – value가 false인지 확인합니다(value === false).


## 구조 분해 할당 (destructuring assignment)

- 원 자료의 요소를 분해, 다른 구조로 변경하기 쉬움
- `let {prop : varName = default, ...rest} = object`
- `let [item1 = default, item2, ...rest] = array`
- `...`로 나머지 요소 가져오기

### 배열 분해 할당

```javascript
// 배열 분해 할당 1)
let arr = ["Bora", "Lee"]
let [firstName, surname] = arr;
alert(firstName); // Bora
alert(surname);  // Lee

// 배열 분해 할당 2)
let [firstName, surname] = "Bora Lee".split(' ');

// 요소 생략
let [firstName, , title] = ["Julius", "Caesar", "Consul", "of the Roman Republic"];
alert( title ); // Consul

// iterable한 자료구조는 모두 가능
let [a, b, c] = "abc"; // ["a", "b", "c"]
let [one, two, three] = new Set([1, 2, 3]);

let user = {};
[user.name, user.surname] = "Bora Lee".split(' ');
alert(user.name); // Bora

let user = {
  name: "John",
  age: 30
};
// 객체의 키와 값 순회하기
for (let [key, value] of Object.entries(user)) {
  alert(`${key}:${value}`); // name:John, age:30이 차례대로 출력
}

// map에서의 destructuring assignment
let user = new Map();
user.set("name", "John");
user.set("age", "30");
for (let [key, value] of user) {
  alert(`${key}:${value}`); // name:John, then age:30
}

// destructuring assignment 사용한 교환
let guest = "Jane";
let admin = "Pete";
// 변수 guest엔 Pete, 변수 admin엔 Jane이 저장되도록 값을 교환함
[guest, admin] = [admin, guest];
alert(`${guest} ${admin}`); // Pete Jane(값 교환이 성공적으로 이뤄졌습니다!)

// '...'로 나머지 요소 가져오기, rest는 배열
let [name1, name2, ...rest] = ["Julius", "Caesar", "Consul", "of the Roman Republic"];
alert(name1); // Julius
alert(name2); // Caesar
alert(rest[0]); // Consul
alert(rest[1]); // of the Roman Republic
alert(rest.length); // 2

// 값이 없을 경우 undefined
let [firstName, surname] = [];
alert(firstName); // undefined
alert(surname); // undefined

// default 값 설정
let [name = "Guest", surname = "Anonymous"] = ["Julius"];
alert(name);    // Julius (배열에서 받아온 값)
alert(surname); // Anonymous (기본값)

// name의 prompt만 실행됨
let [surname = prompt('성을 입력하세요.'), name = prompt('이름을 입력하세요.')] = ["김"];
alert(surname); // 김 (배열에서 받아온 값)
alert(name);    // prompt에서 받아온 값
```

### 객체 분해 할당

```javascript
// 객체 분해 할당
let options = {
  title: "Menu",
  width: 100,
  height: 200
};

let {title, width, height} = options;

alert(title);  // Menu
alert(width);  // 100
alert(height); // 200

// let {...} 안의 순서가 바뀌어도 동일하게 동작함
let {height, width, title} = { title: "Menu", height: 200, width: 100 }

let options = {
  title: "Menu",
  width: 100,
  height: 200
};
// { 객체 프로퍼티: 목표 변수 }
let {width: w, height: h, title} = options;
// width -> w
// height -> h
// title -> title
alert(title);  // Menu
alert(w);      // 100
alert(h);      // 200

// default 설정
let options = {
  title: "Menu"
};
let {width = 100, height = 200, title} = options;
alert(title);  // Menu
alert(width);  // 100
alert(height); // 200

// { 객체 프로퍼티: 목표 변수 } + default
let options = {
  title: "Menu"
};
let {width: w = 100, height: h = 200, title} = options;
alert(title);  // Menu
alert(w);      // 100
alert(h);      // 200

// title만 변수로 뽑아내기
let { title } = options;

// title = 이름이 title인 프로퍼티
// rest = 나머지 프로퍼티들
let {title, ...rest} = options;
// title엔 "Menu", rest엔 {height: 200, width: 100}이 할당됩니다.
alert(rest.height);  // 200
alert(rest.width);   // 100

let title, width, height;
{title, width, height} = {title: "Menu", width: 200, height: 100}; // SyntaxError: Unexpected token '=' 이라는 에러가 아랫줄에서 발생합니다.
({title, width, height} = {title: "Menu", width: 200, height: 100}); // 에러가 발생하지 않습니다.
```

### 중첩 구조 분해(nested destructuring)

```javascript
let options = {
  size: {
    width: 100,
    height: 200
  },
  items: ["Cake", "Donut"],
  extra: true
};

// 코드를 여러 줄에 걸쳐 작성해 의도하는 바를 명확히 드러냄
let {
  size: { // size는 여기,
    width,
    height
  },
  items: [item1, item2], // items는 여기에 할당함
  title = "Menu" // 분해하려는 객체에 title 프로퍼티가 없으므로 기본값을 사용함
} = options;

alert(title);  // Menu
alert(width);  // 100
alert(height); // 200
alert(item1);  // Cake
alert(item2);  // Donut
```

### function argument with destructuring assignment

```javascript
let options = {
  title: "My menu",
  items: ["Item1", "Item2"]
};
function showMenu({
  title = "Untitled",
  width: w = 100,  // width는 w에,
  height: h = 200, // height는 h에,
  items: [item1, item2] // items의 첫 번째 요소는 item1에, 두 번째 요소는 item2에 할당함
}) {
  alert( `${title} ${w} ${h}` ); // My Menu 100 200
  alert( item1 ); // Item1
  alert( item2 ); // Item2
}
showMenu(options);

showMenu({}); // 모든 인수에 기본값이 할당됩니다.
showMenu(); // 에러가 발생할 수 있습니다.
function showMenu({ title = "Menu", width = 100, height = 200 } = {}) {
  alert( `${title} ${width} ${height}` );
}
showMenu(); // Menu 100 200 // 에러 안남
```

## `Promise`

The Promise object represents the eventual completion (or failure) of an asynchronous operation and its resulting value.

> - Producing code와 Consuming code 사이의 비동기적인 처리를 위한 장치
> - 프라미스가 대기 상태일 때, .then/catch/finally 핸들러는 프라미스 완료를 대기
> - 프라미스가 이미 처리상태라면 핸들러가 즉각 실행

- 내부적으로 `state`, `result` 정보 유지
- executor 함수에서 `resolve(Object)` 호출시 `state` => `fulfiled`
- executor 함수에서 `reject(Error)` 호출시 `state` => `rejected`
- 한번 `resolve` 또는 `reject` 호출되면, 재실행 X

```javascript
let promise = new Promise(function(resolve, reject) {
  // 프라미스가 만들어지면 executor 함수는 자동으로 실행
  // 1초 뒤에 일이 성공적으로 끝났다는 신호가 전달되면서 result는 'done'
  setTimeout(() => resolve("done"), 1000);
  // or
  setTimeout(() => reject(new Error("에러 발생!")), 1000);
});
```

### `.then`

Promise가 완료되길 대기하고 resolve, reject를 수행

```javascript
let promise = new Promise(function(resolve, reject) {
  setTimeout(() => reject(new Error("에러 발생!")), 1000);
});

// reject 함수는 .then의 두 번째 함수를 실행합니다.
promise.then(
  result => alert(result), // 실행되지 않음
  error => alert(error) // 1초 후 "Error: 에러 발생!"를 출력
);
```

### `.catch`

`.catch(f)`과 `.then(null,f)` 동일한 동작을 수행하며, 에러를 처리

```javascript
let promise = new Promise((resolve, reject) => {
  setTimeout(() => reject(new Error("에러 발생!")), 1000);
});

// .catch(f)는 promise.then(null, f)과 동일하게 작동합니다
promise.catch(alert); // 1초 뒤 "Error: 에러 발생!" 출력
```

### `.finally`

에러 유무와 관계없이 실행해야 할 작업을 수행

```javascript
new Promise((resolve, reject) => {
  setTimeout(() => resolve("결과"), 2000)
})
  .finally(() => alert("프라미스가 준비되었습니다."))
  .then(result => alert(result)); // <-- .then에서 result를 다룰 수 있음
```

### Promise chaining

Promise chaining이란 promise 대기함수들의 연쇄 실행을 의미한다.
아래와 같이 핸들러를 등록했을 경우에만 Promise 대기함수들을 연속적 실행됨

```javascript
new Promise(function(resolve, reject) {
  setTimeout(() => resolve(1), 1000); // (*)
}).then(function(result) { // (**)
  alert(result); // 1
  return result * 2;
}).then(function(result) { // (***)
  alert(result); // 2
  return result * 2;
}).then(function(result) {
  alert(result); // 4
  return result * 2;
});
```

Promise chaining에서 신규 promise를 반환하여 chaining할 수 있음.

```javascript
new Promise(function(resolve, reject) {
  setTimeout(() => resolve(1), 1000);
}).then(function(result) {
  alert(result); // 1
  return new Promise((resolve, reject) => { // (*)
    setTimeout(() => resolve(result * 2), 1000);
  });
}).then(function(result) { // (**)
  alert(result); // 2
  return new Promise((resolve, reject) => {
    setTimeout(() => resolve(result * 2), 1000);
  });
}).then(function(result) {
  alert(result); // 4
});
```

### `thenable`

`.then`이라는 메서드를 가진 객체는 모두 thenable객체라고 부르며, promise와 같은 방식으로 처리함.

```javascript
class Thenable {
  constructor(num) {
    this.num = num;
  }
  then(resolve, reject) {
    alert(resolve); // function() { 네이티브 코드 }
    // 1초 후 this.num*2와 함께 이행됨
    setTimeout(() => resolve(this.num * 2), 1000); // (**)
  }
}

new Promise(resolve => resolve(1))
  .then(result => {
    return new Thenable(result); // (*)
  })
  .then(alert); // 1000밀리 초 후 2를 보여줌
```

### `fetch`

비동기적으로 추가 정보를 받아오는 동작을 수행함; javascript 내에서 promise를 사용하여 동작함

- AJAX(Asynchronous JavaScript And XML)
- https://ko.javascript.info/fetch

```javascript
let promise = fetch(url, [options]);
```

- url – 접근하고자 하는 URL
- options – 선택 매개변수, method나 header 등을 지정할 수 있음

```javascript
let response = await fetch(url);

if (response.ok) { // HTTP 상태 코드가 200~299일 경우
  // 응답 몬문을 받습니다(관련 메서드는 아래에서 설명).
  let json = await response.json();
} else {
  alert("HTTP-Error: " + response.status);
}
```

### Throw an error in promise

```javascript
// case 1) Throw an error
new Promise((resolve, reject) => {
  throw new Error("에러 발생!");
}).catch(alert); // Error: 에러 발생!

// case 2) Throw an error
new Promise((resolve, reject) => {
  reject(new Error("에러 발생!"));
}).catch(alert); // Error: 에러 발생!

// case 3) Throw an error
new Promise((resolve, reject) => {
  resolve("ok");
}).then((result) => {
  throw new Error("에러 발생!"); // 프라미스가 거부됨
}).catch(alert); // Error: 에러 발생!

// case 4)
// 실행 순서: catch -> then
new Promise((resolve, reject) => {
  throw new Error("에러 발생!");
}).catch(function(error) {
  alert("에러가 잘 처리되었습니다. 정상적으로 실행이 이어집니다.");
}).then(() => alert("다음 핸들러가 실행됩니다."));
```

### Mutilple promises

다수의 promise에 대한 단일 handler 처리

```javascript
Promise.all([
  new Promise(resolve => setTimeout(() => resolve(1), 3000)), // 1
  new Promise(resolve => setTimeout(() => resolve(2), 2000)), // 2
  new Promise(resolve => setTimeout(() => resolve(3), 1000))  // 3
]).then(alert); // 프라미스 전체가 처리되면 1, 2, 3이 반환됩니다. 각 프라미스는 배열을 구성하는 요소가 됩니다.

let urls = [
  'https://api.github.com/users/iliakan',
  'https://api.github.com/users/remy',
  'https://api.github.com/users/jeresig'
];

// fetch를 사용해 url을 프라미스로 매핑합니다.
let requests = urls.map(url => fetch(url));
// Promise.all은 모든 작업이 이행될 때까지 기다립니다.
Promise.all(requests)
  .then(responses => responses.forEach(
    response => alert(`${response.url}: ${response.status}`)
  ));
```

- `Promise.all`: 어느 하나의 promise가 거절되면, reject handler 수행
- `Promise.allSettled`: 모든 promise 처리 대기, 각 promise 상태 반환
  - 응답이 성공할 경우 – `{status:"fulfilled", value:result}`
  - 에러가 발생한 경우 – `{status:"rejected", reason:error}`
- `Promise.race`: 가장 먼저 처리된 promise 결과 반환
- `Promise.resolve/reject`: 많이 안쓰임

### promisify

```javascript
function promisify(f) {
  return function (...args) { // 래퍼 함수를 반환함
    return new Promise((resolve, reject) => {
      function callback(err, result) { // f에 사용할 커스텀 콜백
        if (err) {
          reject(err);
        } else {
          resolve(result);
        }
      }

      args.push(callback); // 위에서 만든 커스텀 콜백을 함수 f의 인수 끝에 추가합니다.

      f.call(this, ...args); // 기존 함수를 호출합니다.
    });
  };
};

let loadScriptPromise = promisify(loadScript);
loadScriptPromise(...).then(...);
```

### Microtask queue

javascript의 비동기 작업을 처리하기 위한 internal work queue로 완료된 promise의 handler를 처리함

- FIFO, first-in-first-out
- 실행될 것이 없을 대 task queue가 작동됨

## `Async` and `Await`

`async`가 함수 앞에 붙으면, 함수는 resolved promise를 반환

```javascript
async function f() {
  // return Promise.resolve(1);와 동일한 결과
  return 1;
}
f().then(alert); // 1
```

- `await`은 `async` 함수 내에서 promise가 이행될 때까지 대기한다.
- `await`은 일반함수에서 사용불가
- promise chaining 대체 사용
- `await`는 `thenable` 객체도 수신
- class에 `await` 선언 가능


```javascript
async function f() {
  let promise = new Promise((resolve, reject) => {
    setTimeout(() => resolve("완료!"), 1000)
  });
  let result = await promise; // 프라미스가 이행될 때까지 기다림 (*)
  alert(result); // "완료!"
}
f();
```

```javascript
(async () => {
  let response = await fetch('/article/promise-chaining/user.json');
  let user = await response.json();
  ...
})();
```

```javascript
// .then 대신 await 사용
async function showAvatar() {
  // JSON 읽기
  let response = await fetch('/article/promise-chaining/user.json');
  let user = await response.json();

  // github 사용자 정보 읽기
  let githubResponse = await fetch(`https://api.github.com/users/${user.name}`);
  let githubUser = await githubResponse.json();

  // 아바타 보여주기
  let img = document.createElement('img');
  img.src = githubUser.avatar_url;
  img.className = "promise-avatar-example";
  document.body.append(img);

  // 3초 대기
  await new Promise((resolve, reject) => setTimeout(resolve, 3000));
  img.remove();
  return githubUser;
}
showAvatar();
```

### async 클래스 메서드

```javascript
class Waiter {
  async wait() {
    return await Promise.resolve(1);
  }
}
new Waiter()
  .wait()
  .then(alert); // 1
```

### `async` & `await` error handling

```javascript
// case 1
async function f() {
  await Promise.reject(new Error("에러 발생!"));
}
// case 2
async function f() {
  throw new Error("에러 발생!");
}

// case 3 - try..catch
async function f() {
  try {
    let response = await fetch('http://유효하지-않은-url');
    let user = await response.json();
  } catch(err) {
    // fetch와 response.json에서 발행한 에러 모두를 여기서 잡습니다.
    alert(err);
  }
}
f();

// case 4
async function f() {
  let response = await fetch('http://유효하지-않은-url');
}
f().catch(alert); // TypeError: failed to fetch // (*)

// case 5 - await Promise.all
// 프라미스 처리 결과가 담긴 배열을 기다립니다.
let results = await Promise.all([
  fetch(url1),
  fetch(url2),
  ...
]);

```

## File Read/Write

https://velog.io/@93jm/Node.js-File-System%EC%9D%98-%EC%82%AC%EC%9A%A9%EA%B3%BC-async-await-%EC%B2%98%EB%A6%AC

## Decorator (wrapping function)

```javascript
function slow(x) {
  // CPU 집약적인 작업이 여기에 올 수 있습니다.
  alert(`slow(${x})을/를 호출함`);
  return x;
}

function cachingDecorator(func) {
  let cache = new Map();
  return function(x) {
    if (cache.has(x)) {    // cache에 해당 키가 있으면
      return cache.get(x); // 대응하는 값을 cache에서 읽어옵니다.
    }

    // let result = func(x);  // 그렇지 않은 경우엔 func를 호출하고,

    let result = func.call(this, x); // 이젠 'this'가 제대로 전달됩니다.

    cache.set(x, result);  // 그 결과를 캐싱(저장)합니다.
    return result;
  };
}

slow = cachingDecorator(slow);
alert( slow(1) ); // slow(1)이 저장되었습니다.
alert( "다시 호출: " + slow(1) ); // 동일한 결과
```

위에서 구현한 캐싱 데코레이터는 `this`가 `undefined`이기 때문에 객체 메서드에 사용하기엔 적합하지 않다.

### 객체의 `call`함수 사용하기

```javascript
function sayHi() {
  alert(this.name);
}
let user = { name: "John" };
let admin = { name: "Admin" };
// call을 사용해 원하는 객체가 'this'가 되도록 합니다.
sayHi.call( user ); // this = John
sayHi.call( admin ); // this = Admin
```

decorator 다시 작성

```javascript
let worker = {
  someMethod() {
    return 1;
  },

  slow(x) {
    alert(`slow(${x})을/를 호출함`);
    return x * this.someMethod(); // (*)
  }
};

function cachingDecorator(func) {
  let cache = new Map();
  return function(x) {
    if (cache.has(x)) {
      return cache.get(x);
    }
    let result = func.call(this, x); // 이젠 'this'가 제대로 전달됩니다.
    cache.set(x, result);
    return result;
  };
}

worker.slow = cachingDecorator(worker.slow); // 캐싱 데코레이터 적용
alert( worker.slow(2) ); // 제대로 동작합니다.
```

## 함수의 호출 함수

- `func.call(context, ...args)` // 전개 문법을 사용해 인수가 담긴 배열을 전달하는 것과
- `func.apply(context, args)`   // call을 사용하는 것은 동일합니다.
- `func.bind(context)` // context (객체)를 func 함수와 연결함. func의 this가 context가 됨

## object property 속성 설정

object의 property는 다음과 같은 속성을 가지며, 수정가능함

- value
- writable
- enumerable
- configurable

```javascript
let user = {};
Object.defineProperty(user, "name", {
  value: "John"
});

let descriptor = Object.getOwnPropertyDescriptor(user, 'name');
alert( JSON.stringify(descriptor, null, 2 ) );
/*
{
  "value": "John",
  "writable": false,
  "enumerable": false,
  "configurable": false
}
 */

// property write 속성 끄기
Object.defineProperty(user, "name", {
  writable: false
});

user.name = "Pete"; // Error: Cannot assign to read only property 'name'
```

## getter, setter properties

```javascript
let obj = {
  get propName() {
    // getter, obj.propName을 실행할 때 실행되는 코드
  },

  set propName(value) {
    // setter, obj.propName = value를 실행할 때 실행되는 코드
  }
  // get, set 둘다 정의되어야 정상 동작
};

// 예제
let user = {
  name: "John",
  surname: "Smith"
};

Object.defineProperty(user, 'fullName', {
  get() {
    return `${this.name} ${this.surname}`;
  },
  set(value) {
    [this.name, this.surname] = value.split(" ");
  }
});

alert(user.fullName); // John Smith
for(let key in user) alert(key); // name, surname
```

## nodejs

### import package

- require: CommonJS 키워드
- import: ES6(ES2015) 키워드

### nodejs getting start

- https://www.nextree.co.kr/p8574/

