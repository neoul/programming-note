class User {
    constructor(name) {
        this.name = name;
        this._age = 11;
    }
    sayHi() {
        console.log("Hi", this.name);
    }
    ['say' + 'Goodbye']() {
        console.log("GoodBye!", this.name);
    }
    // getter
    get age() {
        return this._age;
    }
    // setter
    set age(value) {
        if (value < 18) {
            console.log("Age must be larger than 18.");
            return;
        }
        this._age = value;
        console.log("this._age", this._age);
    }
    // class field
    classfield = "클래스필드";

    nonclassfield_show_age() {
        console.log(this.age);
    }
}

let user = new User("John");
user.sayHi(); // Hi John
user.age = 1; // Age must be larger than 18.
user.sayGoodbye(); // GoodBye! John
console.log(user.classfield, User.classfield); // 클래스필드 undefined; instance에 property 추가
user.nonclassfield_show_age();
setTimeout(user.nonclassfield_show_age, 1000); // undefined; refer이므로 this가 없음