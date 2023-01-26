"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[36694],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var l=a.createContext({}),p=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=p(e.components);return a.createElement(l.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,l=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=p(n),m=r,f=u["".concat(l,".").concat(m)]||u[m]||d[m]||o;return n?a.createElement(f,i(i({ref:t},c),{},{components:n})):a.createElement(f,i({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:r,i[1]=s;for(var p=2;p<o;p++)i[p]=n[p];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},28368:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>u,frontMatter:()=>o,metadata:()=>s,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const o={},i=void 0,s={unversionedId:"examples/psp22",id:"version-1.0.0/examples/psp22",title:"psp22",description:"Overview",source:"@site/versioned_docs/version-1.0.0/examples/psp22.md",sourceDirName:"examples",slug:"/examples/psp22",permalink:"/1.0.0/examples/psp22",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.0.0/examples/psp22.md",tags:[],version:"1.0.0",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"payment-splitter",permalink:"/1.0.0/examples/payment-splitter"},next:{title:"reentrancy-guard",permalink:"/1.0.0/examples/reentrancy-guard"}},l={},p=[{value:"Overview",id:"overview",level:2},{value:"Steps",id:"steps",level:2}],c={toc:p};function u(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"overview"},"Overview"),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22"},"psp22")," token (in the same way you can reuse\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"psp721")," and ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155"},"psp1155"),"). Also, this example shows how you can customize\nthe logic, for example, to reject transfering tokens to ",(0,r.kt)("inlineCode",{parentName:"p"},"hated_account"),"."),(0,r.kt)("h2",{id:"steps"},"Steps"),(0,r.kt)("ol",null,(0,r.kt)("li",{parentName:"ol"},"Include dependencies to ",(0,r.kt)("inlineCode",{parentName:"li"},"psp22")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"brush")," in the cargo file.")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-markdown"},'[dependencies]\nink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }\nink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }\nink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }\nink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }\nink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }\nink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }\n\nscale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }\nscale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\npsp22 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\nbrush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\n\n[features]\ndefault = ["std"]\nstd = [\n   "ink_primitives/std",\n   "ink_metadata",\n   "ink_metadata/std",\n   "ink_env/std",\n   "ink_storage/std",\n   "ink_lang/std",\n   "scale/std",\n   "scale-info",\n   "scale-info/std",\n\n   # These dependencies   \n   "psp22/std",\n   "brush/std",\n]\n')),(0,r.kt)("ol",{start:2},(0,r.kt)("li",{parentName:"ol"},"Replace ",(0,r.kt)("inlineCode",{parentName:"li"},"ink::contract")," macro by ",(0,r.kt)("inlineCode",{parentName:"li"},"brush::contract"),".\nImport ",(0,r.kt)("strong",{parentName:"li"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"li"},"psp22::traits"),".")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[brush::contract]\npub mod my_psp22 {\n   use psp22::traits::*;\n   use ink_storage::Lazy;\n   use ink_prelude::{string::String, vec::Vec};\n")),(0,r.kt)("ol",{start:3},(0,r.kt)("li",{parentName:"ol"},"Declare storage struct and declare the fields related to ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22Storage")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22MetadataStorage"),"\ntraits. Then you need to derive ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22Storage")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22MetadataStorage")," traits and mark corresponding fields\nwith ",(0,r.kt)("inlineCode",{parentName:"li"},"#[PSP22StorageField]")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"#[PSP22MetadataStorageField]")," attributes. Deriving these traits allows you to reuse\nthe default implementation of ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22Metadata"),".")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PSP22Storage, PSP22MetadataStorage)]\npub struct MyPSP22 {\n    #[PSP22StorageField]\n    psp22: PSP22Data,\n    #[PSP22MetadataStorageField]\n    metadata: PSP22MetadataData,\n}\n")),(0,r.kt)("ol",{start:4},(0,r.kt)("li",{parentName:"ol"},"Inherit implementations of ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22Metadata")," traits. You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"li"},"impl")," block.")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP22 for MyPSP22 {}\n\nimpl PSP22Metadata for MyPSP22 {}\n")),(0,r.kt)("ol",{start:5},(0,r.kt)("li",{parentName:"ol"},"Define constructor. Your basic version of ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22")," contract is ready!")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyPSP22 {\n   #[ink(constructor)]\n   pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n      let mut instance = Self::default();\n      Lazy::set(&mut instance.metadata.name, name);\n      Lazy::set(&mut instance.metadata.symbol,symbol);\n      Lazy::set(&mut instance.metadata.decimals,decimal);\n      instance._mint(instance.env().caller(), _total_supply);\n      instance\n   }\n}\n")),(0,r.kt)("ol",{start:6},(0,r.kt)("li",{parentName:"ol"},"Customize it by adding hated account logic. It will contain two public methods ",(0,r.kt)("inlineCode",{parentName:"li"},"set_hated_account")," and ",(0,r.kt)("inlineCode",{parentName:"li"},"get_hated_account"),". Also we will\noverride ",(0,r.kt)("inlineCode",{parentName:"li"},"_before_token_transfer")," method in ",(0,r.kt)("inlineCode",{parentName:"li"},"PSP22")," implementation. And we will add the ",(0,r.kt)("inlineCode",{parentName:"li"},"hated_account: AccountId")," field to the structure.")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#[ink(storage)]\n#[derive(Default, PSP22Storage, PSP22MetadataStorage)]\npub struct MyPSP22 {\n   #[PSP22StorageField]\n   psp22: PSP22Data,\n   #[PSP22MetadataStorageField]\n   metadata: PSP22MetadataData,\n   // fields for hater logic\n   hated_account: AccountId,\n}\n\nimpl PSP22 for MyPSP22 {\n   // Let\'s override method to reject transactions to bad account\n   fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {\n      assert!(_to != self.hated_account, "{}", PSP22Error::Custom(String::from("I hate this account!")).as_ref());\n   }\n}\n\nimpl PSP22Metadata for MyPSP22 {}\n\nimpl MyPSP22 {\n   #[ink(constructor)]\n   pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n      let mut instance = Self::default();\n      Lazy::set(&mut instance.metadata.name, name);\n      Lazy::set(&mut instance.metadata.symbol,symbol);\n      Lazy::set(&mut instance.metadata.decimals,decimal);\n      instance._mint(instance.env().caller(), _total_supply);\n      instance\n   }\n\n   #[ink(message)]\n   pub fn set_hated_account(&mut self, hated: AccountId) {\n      self.hated_account = hated;\n   }\n\n   #[ink(message)]\n   pub fn get_hated_account(&self) -> AccountId {\n      self.hated_account.clone()\n   }\n}\n')))}u.isMDXComponent=!0}}]);