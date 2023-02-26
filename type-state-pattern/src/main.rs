#![allow(unused)]

use std::collections::HashMap;

// problems
// 현재 구현의 문제는 사용자가 API를 오용하지 못하도록 방지하는 것이 없다는 것입니다.
// 예를 들어, 사용자는 함수를 순서대로 호출하거나 사용 가능한 상태가 아닌 메서드를 호출할 수 있습니다.
// 해결책은 암호 관리자가 잠금 상태와 잠금 해제 상태를 갖는 API를 인코딩하고 암호 관리자에 추가적인 상태를 추가하는 것입니다.
// solution 1.
// "locked"라는 새 필드를 암호 관리자에 추가하거나 함수의 시그니처를 변경하여 결과 유형을 반환하거나 잠금 상태와
// 잠금 해제 상태를 갖는 두 개의 별도 구조체를 가지는 등 몇 가지 솔루션을 소개합니다.
// solution 2.
// 최종적인 솔루션은 두 개의 별도 구조체를 갖는 것입니다.
// 각 구조체에는 자체 구현 블록이 있습니다.
// 잠긴 암호 관리자에는 생성자 함수와 잠금 해제 메서드가 포함되며, 잠긴 암호 관리자에서는 암호화 및 버전 메서드도 포함됩니다.
// 잠금 해제된 암호 관리자에는 잠금, 비밀번호 목록을 나열하는 메서드, 비밀번호를 추가하는 메서드 등이 포함됩니다.
// 잠금 해제된 암호 관리자에서도 암호화 및 버전 메서드가 포함됩니다.
// 업데이트된 API는 현재 상태에서 사용할 수 없는 메서드에 접근하지 못하도록 사용자를 방지합니다.
// 사용자는 잠금 상태나 잠금 해제 상태에서 의미 있는 메서드에만 액세스할 수 있습니다.
// 그러나 두가지 문제점이 있습니다.
// 하나는 필드가 중복이 된다는 것이고, 두 개의 별도 구조체가 sync를 맞추는 작업이 필요하다는 점 입니다.
// generic을 통해서 문제를 해결해봅시다.
// solution3.

#[derive(Debug)]
struct Locked;

#[derive(Debug)]
struct Unlocked;

#[derive(Debug)]
struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: std::marker::PhantomData<State>, // Allowed two distinct state, PhantomData is zero-sized type.
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: &String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass: master_pass.to_string(),
            passwords: self.passwords,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: std::marker::PhantomData::<Locked>,
        }
    }
    pub fn list_passwords(&mut self) -> &HashMap<String, String> {
        &self.passwords
    }
    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

// 모든 state에서 사용 가능한 패턴
impl<State> PasswordManager<State> {
    pub fn encryption(&self) -> String {
        todo!()
    }
    pub fn version(&self) -> String {
        todo!()
    }
}

impl PasswordManager {
    pub fn new(master_pass: &String) -> Self {
        Self {
            master_pass: master_pass.to_string(),
            passwords: Default::default(),
            state: Default::default(),
        }
    }
}

fn main() {
    let master_pass = "master123".to_string();
    let mut manager = PasswordManager::new(&master_pass);
    let mut manager: PasswordManager<Unlocked> = manager.unlock(&master_pass);
    manager.list_passwords();
    manager.lock();
}