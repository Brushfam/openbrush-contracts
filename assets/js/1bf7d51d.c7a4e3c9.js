"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[37874],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var r=n(67294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var i=r.createContext({}),s=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},p=function(e){var t=s(e.components);return r.createElement(i.Provider,{value:t},e.children)},m="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,i=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),m=s(n),d=o,f=m["".concat(i,".").concat(d)]||m[d]||u[d]||a;return n?r.createElement(f,l(l({ref:t},p),{},{components:n})):r.createElement(f,l({ref:t},p))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,l=new Array(a);l[0]=d;var c={};for(var i in t)hasOwnProperty.call(t,i)&&(c[i]=t[i]);c.originalType=e,c[m]="string"==typeof e?e:o,l[1]=c;for(var s=2;s<a;s++)l[s]=n[s];return r.createElement.apply(null,l)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},77630:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>l,default:()=>u,frontMatter:()=>a,metadata:()=>c,toc:()=>s});var r=n(87462),o=(n(67294),n(3905));const a={sidebar_position:8,title:"Timelock Controller"},l=void 0,c={unversionedId:"smart-contracts/timelock-controller",id:"version-3.0.0-beta/smart-contracts/timelock-controller",title:"Timelock Controller",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-3.0.0-beta/smart-contracts/timelock-controller.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/timelock-controller",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/timelock-controller",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.0.0-beta/smart-contracts/timelock-controller.md",tags:[],version:"3.0.0-beta",sidebarPosition:8,frontMatter:{sidebar_position:8,title:"Timelock Controller"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Token Timelock",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Utils/token-timelock"},next:{title:"PSP22 Pallet",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22-Pallet/"}},i={},s=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Define constructor",id:"step-2-define-constructor",level:2},{value:"Final code",id:"final-code",level:2}],p={toc:s},m="wrapper";function u(e){let{components:t,...n}=e;return(0,o.kt)(m,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,o.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller"},"timelock-controller"),"."),(0,o.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,o.kt)("p",null,"With ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,o.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to import the ",(0,o.kt)("inlineCode",{parentName:"p"},"timelock_controller")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"access_controll")," modules, enable corresponding features, and embed modules data structures\nas described in ",(0,o.kt)("a",{parentName:"p",href:"/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,o.kt)("p",null,"The main traits are ",(0,o.kt)("inlineCode",{parentName:"p"},"AccessControl")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"TimelockController"),"."),(0,o.kt)("h2",{id:"step-2-define-constructor"},"Step 2: Define constructor"),(0,o.kt)("p",null,"Define constructor where you init admin of the contract."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    #[ink(constructor)]\n    pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {\n        let mut instance = Self::default();\n\n        let caller = Self::env().caller();\n        // `TimelockController` and `AccessControl` have `_init_with_admin` methods.\n        // You need to call it for each trait separately, to initialize everything for these traits.\n        access_control::Internal::_init_with_admin(instance, caller);\n        timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);\n        \n        instance\n    }\n}\n")),(0,o.kt)("h2",{id:"final-code"},"Final code"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_timelock_controller {\n    use ink::prelude::vec::Vec;\n    use openbrush::{\n        contracts::timelock_controller::*,\n        traits::Storage,\n    };\n\n    #[ink(storage)]\n    #[derive(Default, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        access_control: access_control::Data,\n        #[storage_field]\n        timelock: timelock_controller::Data,\n    }\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {\n           let mut instance = Self::default();\n\n            let caller = Self::env().caller();\n            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.\n            // You need to call it for each trait separately, to initialize everything for these traits.\n            access_control::Internal::_init_with_admin(instance, caller);\n            timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);\n\n            instance\n        }\n    }\n\n    // `TimelockController` is an extension for `AccessControl`, so you have to inherit logic related to both modules.\n    impl AccessControl for Contract {}\n    impl TimelockController for Contract {}\n}\n')),(0,o.kt)("p",null,"You can check an example of the usage of ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/timelock_controller"},"TimelockController"),"."))}u.isMDXComponent=!0}}]);