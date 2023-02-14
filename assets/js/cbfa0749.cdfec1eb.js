"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[74106],{3905:(e,t,n)=>{n.d(t,{Zo:()=>s,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),c=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},s=function(e){var t=c(e.components);return a.createElement(p.Provider,{value:t},e.children)},d="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,s=o(e,["components","mdxType","originalType","parentName"]),d=c(n),m=r,f=d["".concat(p,".").concat(m)]||d[m]||u[m]||i;return n?a.createElement(f,l(l({ref:t},s),{},{components:n})):a.createElement(f,l({ref:t},s))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,l=new Array(i);l[0]=m;var o={};for(var p in t)hasOwnProperty.call(t,p)&&(o[p]=t[p]);o.originalType=e,o[d]="string"==typeof e?e:r,l[1]=o;for(var c=2;c<i;c++)l[c]=n[c];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},808:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>l,default:()=>u,frontMatter:()=>i,metadata:()=>o,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const i={sidebar_position:4,title:"Reentrancy Guard"},l=void 0,o={unversionedId:"smart-contracts/reentrancy-guard",id:"version-1.2.0/smart-contracts/reentrancy-guard",title:"Reentrancy Guard",description:"This example shows how you can use the nonreentrant",source:"@site/versioned_docs/version-1.2.0/smart-contracts/reentrancy-guard.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/reentrancy-guard",permalink:"/openbrush-contracts/1.2.0/smart-contracts/reentrancy-guard",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.2.0/smart-contracts/reentrancy-guard.md",tags:[],version:"1.2.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Reentrancy Guard"},sidebar:"tutorialSidebar",previous:{title:"Ownable",permalink:"/openbrush-contracts/1.2.0/smart-contracts/ownable"},next:{title:"Pausable",permalink:"/openbrush-contracts/1.2.0/smart-contracts/pausable"}},p={},c=[{value:"MyFlipper",id:"myflipper",level:2},{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:3},{value:"Step 2: Add imports",id:"step-2-add-imports",level:3},{value:"Step 3: Define storage",id:"step-3-define-storage",level:3},{value:"Step 4: Add modifiers",id:"step-4-add-modifiers",level:3},{value:"Step 5: Add stub contract",id:"step-5-add-stub-contract",level:3},{value:"FlipOnMe",id:"fliponme",level:2},{value:"Step 1: Define <code>FlipOnMe</code> contract",id:"step-1-define-fliponme-contract",level:3},{value:"Step 2: Include dependencies",id:"step-2-include-dependencies",level:3},{value:"Testing",id:"testing",level:2}],s={toc:c},d="wrapper";function u(e){let{components:t,...n}=e;return(0,r.kt)(d,(0,a.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can use the ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/security/reentrancy_guard"},"non_reentrant"),"\nmodifier to prevent reentrancy into certain functions. In this example we will create two contracts:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"my_flipper_guard")," - this contract is the simple version of ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/paritytech/ink/tree/master/examples/flipper"},"flipper"),",\nbut method ",(0,r.kt)("inlineCode",{parentName:"li"},"flip")," will be marked with ",(0,r.kt)("inlineCode",{parentName:"li"},"non_reentrant")," modifier, and we will add additional method, also marked\nwith ",(0,r.kt)("inlineCode",{parentName:"li"},"non_reentrant"),", which will ask another contract to call ",(0,r.kt)("inlineCode",{parentName:"li"},"flip")," of our ",(0,r.kt)("inlineCode",{parentName:"li"},"flipper"),"."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"flip_on_me")," is a contract which has the only one method ",(0,r.kt)("inlineCode",{parentName:"li"},"flip_on_me"),". This method will try to call ",(0,r.kt)("inlineCode",{parentName:"li"},"flip")," on the caller\n(it means that caller must be a contract with method ",(0,r.kt)("inlineCode",{parentName:"li"},"flip"),").")),(0,r.kt)("h2",{id:"myflipper"},"MyFlipper"),(0,r.kt)("h3",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Reentrancy Guard via ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["reentrancy_guard"] }\n')),(0,r.kt)("h3",{id:"step-2-add-imports"},"Step 2: Add imports"),(0,r.kt)("p",null,"To declare the contract, you need to use ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything"),"\nfrom ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contracts::reentrancy_guard"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n\n#[brush::contract]\npub mod my_flipper_guard {\n  use brush::{\n    contracts::reentrancy_guard::*,\n    modifiers,\n  };\n\n  use crate::flip_on_me::CallerOfFlip;\n  use ink_env::call::FromAccountId;\n')),(0,r.kt)("h3",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field for ",(0,r.kt)("inlineCode",{parentName:"p"},"ReentrancyGuardStorage")," trait. Then you need to\nderive ",(0,r.kt)("inlineCode",{parentName:"p"},"ReentrancyGuardStorage")," trait and mark the field with ",(0,r.kt)("inlineCode",{parentName:"p"},"#[ReentrancyGuardStorageField]")," attribute. Deriving\nthis trait allows you to use ",(0,r.kt)("inlineCode",{parentName:"p"},"non_reentrant")," modifier."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, ReentrancyGuardStorage)]\npub struct MyFlipper {\n    #[ReentrancyGuardStorageField]\n    guard: ReentrancyGuardData,\n    value: bool,\n}\n")),(0,r.kt)("h3",{id:"step-4-add-modifiers"},"Step 4: Add modifiers"),(0,r.kt)("p",null,"After that you can add ",(0,r.kt)("inlineCode",{parentName:"p"},"non_reentrant")," modifier to ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"call_flip_on_me")," methods."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyFlipper {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        Self::default()\n    }\n\n    #[ink(message)]\n    pub fn get_value(&self) -> bool {\n        self.value\n    }\n\n    #[ink(message)]\n    #[brush::modifiers(non_reentrant)]\n    pub fn flip(&mut self) {\n        self.value = !self.value;\n    }\n\n    #[ink(message)]\n    #[modifiers(non_reentrant)]\n    pub fn call_flip_on_me(&mut self, callee: AccountId) {\n        // This method will do a cross-contract call to callee account. It calls method `flip_on_me`.\n        // Callee contract during execution of `flip_on_me` will call `flip` of this contract.\n        // `call_flip_on_me` and `flip` are marked with `non_reentrant` modifier. It means,\n        // that call of `flip` after `call_flip_on_me` must fail.\n        let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);\n        flipper.flip_on_me();\n    }\n}\n")),(0,r.kt)("h3",{id:"step-5-add-stub-contract"},"Step 5: Add stub contract"),(0,r.kt)("p",null,"To simplify cross contract call to ",(0,r.kt)("inlineCode",{parentName:"p"},"FlipOnMe")," contract let's create a wrapper around the contract's account id.\nFor that, we will define another contract in this crate with ",(0,r.kt)("inlineCode",{parentName:"p"},"#[ink_lang::contract(compile_as_dependency = true)]"),"\nand empty methods but with the same signature as in the original contract."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"/// This is a stub implementation of contract with method `flip_on_me`.\n/// We need this implementation to create a wrapper around account id of contract.\n/// With this wrapper, we can easily call methods of some contract.\n/// Example:\n/// ```\n/// let mut flipper: CallerOfFlip = FromAccountId::from_account_id(callee);\n/// flipper.flip_on_me();\n/// ```\n#[ink_lang::contract(compile_as_dependency = true)]\npub mod flip_on_me {\n    #[ink(storage)]\n    pub struct CallerOfFlip {}\n\n    impl CallerOfFlip {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            unimplemented!()\n        }\n    }\n\n    impl CallerOfFlip {\n        #[ink(message)]\n        pub fn flip_on_me(&mut self) {\n            unimplemented!()\n        }\n    }\n}\n")),(0,r.kt)("h2",{id:"fliponme"},"FlipOnMe"),(0,r.kt)("p",null,"It's a simple contract that doesn't use any logic from the OpenBrush, so you can use simple ink! here."),(0,r.kt)("h3",{id:"step-1-define-fliponme-contract"},"Step 1: Define ",(0,r.kt)("inlineCode",{parentName:"h3"},"FlipOnMe")," contract"),(0,r.kt)("p",null,"It has the only method ",(0,r.kt)("inlineCode",{parentName:"p"},"flip_on_me"),", which will call ",(0,r.kt)("inlineCode",{parentName:"p"},"flip")," on caller."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink_lang::contract]\npub mod flip_on_me {\n    use ink_env::call::FromAccountId;\n    use my_flipper_guard::my_flipper_guard::MyFlipper;\n\n    #[ink(storage)]\n    #[derive(Default)]\n    pub struct FlipOnMe {}\n\n    impl FlipOnMe {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            Self::default()\n        }\n\n        #[ink(message)]\n        pub fn flip_on_me(&mut self) {\n            let caller = self.env().caller();\n            // This method does a cross-contract call to caller contract and calls the `flip` method.\n            let mut flipper: MyFlipper = FromAccountId::from_account_id(caller);\n            flipper.flip();\n        }\n    }\n}\n")),(0,r.kt)("h3",{id:"step-2-include-dependencies"},"Step 2: Include dependencies"),(0,r.kt)("p",null,"To do a cross-contract call to ",(0,r.kt)("inlineCode",{parentName:"p"},"MyFlipper")," you need to import the ",(0,r.kt)("inlineCode",{parentName:"p"},"MyFlipper")," contract with ",(0,r.kt)("inlineCode",{parentName:"p"},"ink-as-dependency")," feature."),(0,r.kt)("blockquote",null,(0,r.kt)("p",{parentName:"blockquote"},(0,r.kt)("strong",{parentName:"p"},(0,r.kt)("em",{parentName:"strong"},"Note:")),"  The crate type of the ",(0,r.kt)("inlineCode",{parentName:"p"},"MyFlipper")," should be ",(0,r.kt)("inlineCode",{parentName:"p"},"rlib")," for that.")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'[dependencies]\nink_primitives = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_metadata = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }\nink_env = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_storage = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_lang = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_prelude = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\n\nscale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }\nscale-info = { version = "1", default-features = false, features = ["derive"], optional = true }\n\n# This dependencies\nmy_flipper_guard = { path = "../flipper", default - features = false, features = ["ink-as-dependency"] }\n\n[features]\ndefault = ["std"]\nstd = [\n    "ink_primitives/std",\n    "ink_metadata",\n    "ink_metadata/std",\n    "ink_env/std",\n    "ink_storage/std",\n    "ink_lang/std",\n    "scale/std",\n    "scale-info",\n    "scale-info/std",\n    \n    # This dependencies\n    "my_flipper_guard/std",\n]\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/reentrancy_guard"},"ReentrancyGuard"),"."),(0,r.kt)("h2",{id:"testing"},"Testing"),(0,r.kt)("p",null,"For testing, you can run the ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/blob/main/tests/reentrancy-guard.tests.ts"},"integration test"),", or you can deploy both\ncontracts and call ",(0,r.kt)("inlineCode",{parentName:"p"},"call_flip_on_me")," on ",(0,r.kt)("inlineCode",{parentName:"p"},"MyFlipper"),"\naccount providing account id of ",(0,r.kt)("inlineCode",{parentName:"p"},"FlipOnMe")," contract as an argument."))}u.isMDXComponent=!0}}]);