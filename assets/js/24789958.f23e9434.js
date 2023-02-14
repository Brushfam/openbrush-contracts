"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[56591],{3905:(t,e,a)=>{a.d(e,{Zo:()=>c,kt:()=>k});var n=a(67294);function o(t,e,a){return e in t?Object.defineProperty(t,e,{value:a,enumerable:!0,configurable:!0,writable:!0}):t[e]=a,t}function r(t,e){var a=Object.keys(t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(t);e&&(n=n.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),a.push.apply(a,n)}return a}function l(t){for(var e=1;e<arguments.length;e++){var a=null!=arguments[e]?arguments[e]:{};e%2?r(Object(a),!0).forEach((function(e){o(t,e,a[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(a)):r(Object(a)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(a,e))}))}return t}function i(t,e){if(null==t)return{};var a,n,o=function(t,e){if(null==t)return{};var a,n,o={},r=Object.keys(t);for(n=0;n<r.length;n++)a=r[n],e.indexOf(a)>=0||(o[a]=t[a]);return o}(t,e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);for(n=0;n<r.length;n++)a=r[n],e.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(t,a)&&(o[a]=t[a])}return o}var s=n.createContext({}),p=function(t){var e=n.useContext(s),a=e;return t&&(a="function"==typeof t?t(e):l(l({},e),t)),a},c=function(t){var e=p(t.components);return n.createElement(s.Provider,{value:e},t.children)},u="mdxType",d={inlineCode:"code",wrapper:function(t){var e=t.children;return n.createElement(n.Fragment,{},e)}},m=n.forwardRef((function(t,e){var a=t.components,o=t.mdxType,r=t.originalType,s=t.parentName,c=i(t,["components","mdxType","originalType","parentName"]),u=p(a),m=o,k=u["".concat(s,".").concat(m)]||u[m]||d[m]||r;return a?n.createElement(k,l(l({ref:e},c),{},{components:a})):n.createElement(k,l({ref:e},c))}));function k(t,e){var a=arguments,o=e&&e.mdxType;if("string"==typeof t||o){var r=a.length,l=new Array(r);l[0]=m;var i={};for(var s in e)hasOwnProperty.call(e,s)&&(i[s]=e[s]);i.originalType=t,i[u]="string"==typeof t?t:o,l[1]=i;for(var p=2;p<r;p++)l[p]=a[p];return n.createElement.apply(null,l)}return n.createElement.apply(null,a)}m.displayName="MDXCreateElement"},18976:(t,e,a)=>{a.r(e),a.d(e,{assets:()=>s,contentTitle:()=>l,default:()=>d,frontMatter:()=>r,metadata:()=>i,toc:()=>p});var n=a(87462),o=(a(67294),a(3905));const r={sidebar_position:4,title:"Deployment",sidebar_label:"Deployment"},l=void 0,i={unversionedId:"deployment",id:"version-2.2.0/deployment",title:"Deployment",description:"- Deployment of ink! based smart contracts",source:"@site/versioned_docs/version-2.2.0/deployment.md",sourceDirName:".",slug:"/deployment",permalink:"/openbrush-contracts/2.2.0/deployment",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-2.2.0/deployment.md",tags:[],version:"2.2.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Deployment",sidebar_label:"Deployment"},sidebar:"tutorialSidebar",previous:{title:"Notes about methods",permalink:"/openbrush-contracts/2.2.0/smart-contracts/example/implementation"},next:{title:"EVM vs WASM Smart Contracts",permalink:"/openbrush-contracts/2.2.0/evm-wasm-smart-contracts"}},s={},p=[{value:"Ecosystem",id:"ecosystem",level:3},{value:"Overview",id:"overview",level:3},{value:"Build",id:"build",level:3},{value:"Install polkadot extention for your browser and create account",id:"install-polkadot-extention-for-your-browser-and-create-account",level:3},{value:"Deployment on local network",id:"deployment-on-local-network",level:3},{value:"Call the smart contract",id:"call-the-smart-contract",level:3},{value:"Rococo Faucet",id:"rococo-faucet",level:3},{value:"Deploy to Canvas",id:"deploy-to-canvas",level:3},{value:"Astar",id:"astar",level:3},{value:"Deploy to Shibuya",id:"deploy-to-shibuya",level:3}],c={toc:p},u="wrapper";function d(t){let{components:e,...r}=t;return(0,o.kt)(u,(0,n.Z)({},c,r,{components:e,mdxType:"MDXLayout"}),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Deployment of ink! based smart contracts")),(0,o.kt)("p",null,"This document contains description of how to deploy and test smart contracts locally and in testnet."),(0,o.kt)("h3",{id:"ecosystem"},"Ecosystem"),(0,o.kt)("p",null,"Polkadot doesn't support smart contract execution, only parachains can provide this functionality. More information\nabout how it works you can find on ",(0,o.kt)("a",{parentName:"p",href:"https://wiki.polkadot.network/docs/en/build-smart-contracts"},"official wiki"),"."),(0,o.kt)("p",null,"The list of standalone blockchain/parachains that support ink! smart contracts:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://astar.network/"},"Astar"))),(0,o.kt)("h3",{id:"overview"},"Overview"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"To deploy contract you should build your own contract or get some example from ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples"},"Openbrush"),". You can find instruction how to build ink! smart contract in ",(0,o.kt)("a",{parentName:"p",href:"https://ink.substrate.io/getting-started/building-your-contract"},"docs"))),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"You have to choose substrate network to deploy your contract.\nThere are several option you have:"),(0,o.kt)("ul",{parentName:"li"},(0,o.kt)("li",{parentName:"ul"},"Local substrate node with pallet contracts."),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"Canvas")," network"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"Shibuya")," - Astar testnet"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"Shiden")," - Astar canary network"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"Astar")," main network (will support pallet contracts in near futures)"),(0,o.kt)("li",{parentName:"ul"},"Other networks which supports pallet contracts"))),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"Be sure that you have installed ",(0,o.kt)("inlineCode",{parentName:"p"},"polkadot.js.org")," ",(0,o.kt)("a",{parentName:"p",href:"#install-polkadot-extention-for-your-browser-and-create-account"},"wallet")," extenstion for your browser")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"Here you can find how to ",(0,o.kt)("a",{parentName:"p",href:"https://ink.substrate.io/cargo-contract-cli/#usage"},"Build")," ",(0,o.kt)("strong",{parentName:"p"},"ink!")," smart contract")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"Let's ",(0,o.kt)("a",{parentName:"p",href:"#deployment-on-local-network"},"deploy to local network"))),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"You can manuly ",(0,o.kt)("a",{parentName:"p",href:"#call-the-smart-contract"},"call")," our deployed contract")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},(0,o.kt)("a",{parentName:"p",href:"https://github.com/paritytech/cumulus#canvas-"},"Canvas")," - a Smart Contracts ",(0,o.kt)("a",{parentName:"p",href:"https://wiki.polkadot.network/docs/learn-parachains"},"Parachain")," which was deployed on ",(0,o.kt)("a",{parentName:"p",href:"https://polkadot.network/ru/"},"Polkadot")," test network - ",(0,o.kt)("a",{parentName:"p",href:"https://polkadot.network/tag/rococo/"},"Rococo"),". You need to get free ",(0,o.kt)("inlineCode",{parentName:"p"},"ROC")," token using ",(0,o.kt)("a",{parentName:"p",href:"#rococo-faucet"},"faucet")," to deploy contract to Canvas network. Finally deploy your ink! smart contract to ",(0,o.kt)("a",{parentName:"p",href:"#deploy-to-=anvas"},"canvas"))),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},(0,o.kt)("a",{parentName:"p",href:"#astar"},"Astar")," - ",(0,o.kt)("a",{parentName:"p",href:"https://webassembly.org/"},"WASM")," + ",(0,o.kt)("a",{parentName:"p",href:"https://ethereum.org/en/developers/docs/evm/"},"EVM")," Hub on ",(0,o.kt)("a",{parentName:"p",href:"https://polkadot.network/"},"Polkadot"),". More info about astar ",(0,o.kt)("a",{parentName:"p",href:"https://docs.astar.network/"},"here"))),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"You can deploy ",(0,o.kt)("strong",{parentName:"p"},"ink!")," smart contract to ",(0,o.kt)("a",{parentName:"p",href:"#deploy-to-shibuya"},"Shibuya")," (astar test network). How to get free ",(0,o.kt)("inlineCode",{parentName:"p"},"SBY")," using ",(0,o.kt)("a",{parentName:"p",href:"https://docs.astar.network/integration/testnet-faucet"},"faucet")))),(0,o.kt)("h3",{id:"build"},"Build"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"navigate to ",(0,o.kt)("inlineCode",{parentName:"li"},"./openbrush/examples/psp22")),(0,o.kt)("li",{parentName:"ul"},"build ink! contract using:")),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre"},"cargo +nightly contract build\n")),(0,o.kt)("p",null,"Once the compilation is completed, a target folder is created. In this folder, under the ink subdirectory, you will be able to see a ",(0,o.kt)("inlineCode",{parentName:"p"},"my_psp22.wasm")," file and a ",(0,o.kt)("inlineCode",{parentName:"p"},"metadata.json")," file. ",(0,o.kt)("inlineCode",{parentName:"p"},"my_psp22.wasm")," is your contract that has been compiled to web assembly and the ",(0,o.kt)("inlineCode",{parentName:"p"},"metadata.json")," is a JSON abstraction of your contract."),(0,o.kt)("p",null,"You will find 3 files in folder ",(0,o.kt)("inlineCode",{parentName:"p"},"./openbrush/examples/psp22/target/ink")),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"my_psp22.contract")," (code + metadata)"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"my_psp22.wasm")," (the contract\u2019s code)"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("inlineCode",{parentName:"li"},"metadata.json")," (the contract\u2019s metadata)")),(0,o.kt)("h3",{id:"install-polkadot-extention-for-your-browser-and-create-account"},"Install polkadot extention for your browser and create account"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Navigate to ",(0,o.kt)("a",{parentName:"li",href:"https://polkadot.js.org/extension/"},"Polkadot.js.org")," extention tab and install to your browser. You need wallet extention to sign and submit transaction for deployment contract and manual testing via UI"),(0,o.kt)("li",{parentName:"ul"},"Create or import polkadot account. You need account and some tokens on that account to deploy and test contracts on test network like ",(0,o.kt)("inlineCode",{parentName:"li"},"Canvas"),", ",(0,o.kt)("inlineCode",{parentName:"li"},"Shibuya")," or main network like ",(0,o.kt)("inlineCode",{parentName:"li"},"Shiden")," and ",(0,o.kt)("inlineCode",{parentName:"li"},"Astar")," in near futures. How to get free tokens for test net you will find ",(0,o.kt)("a",{parentName:"li",href:"#rococo-Faucet"},"there")),(0,o.kt)("li",{parentName:"ul"},"Please write down your wallet's mnemonic seed and keep it in a safe place. The mnemonic can be used to restore your wallet. Keep it carefully to not lose your assets.")),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(80907).Z,width:"1792",height:"918"})),(0,o.kt)("h3",{id:"deployment-on-local-network"},"Deployment on local network"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Substrate framework pre requisites ",(0,o.kt)("a",{parentName:"li",href:"https://ink.substrate.io/getting-started/setup/#substrate-framework-pre-requisites"},"guide")),(0,o.kt)("li",{parentName:"ul"},"Run a Substrate Node ",(0,o.kt)("a",{parentName:"li",href:"https://ink.substrate.io/getting-started/running-substrate"},"guide")),(0,o.kt)("li",{parentName:"ul"},"Navigate to the ",(0,o.kt)("a",{parentName:"li",href:"https://polkadot.js.org"},"Polkadot.js.org")," in a web browser"),(0,o.kt)("li",{parentName:"ul"},"Verify that you are connected to the ",(0,o.kt)("a",{parentName:"li",href:"https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer"},"Local Node"),".")),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(98687).Z,width:"1790",height:"876"})),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},"Upload and deploy contract"),(0,o.kt)("p",{parentName:"li"},"Click ",(0,o.kt)("inlineCode",{parentName:"p"},"Developer")," -> ",(0,o.kt)("inlineCode",{parentName:"p"},"Contracts")," -> ",(0,o.kt)("inlineCode",{parentName:"p"},"Upload & deploy code"),". Specify the user account to use for deployment. Any fees will be deducted from deployment account. Upload ",(0,o.kt)("inlineCode",{parentName:"p"},"*.contract")," file. ",(0,o.kt)("inlineCode",{parentName:"p"},"*.contract")," file contains the ",(0,o.kt)("inlineCode",{parentName:"p"},"ABI")," for the ",(0,o.kt)("inlineCode",{parentName:"p"},"WASM")," code. The ",(0,o.kt)("inlineCode",{parentName:"p"},"ABI")," is required and stored for future operations such as sending messages. Type a descriptive name for the smart contract. Set value ",(0,o.kt)("inlineCode",{parentName:"p"},"1000")," for ",(0,o.kt)("inlineCode",{parentName:"p"},"totalSupply")," when initialize the contract using constructor. And finally click ",(0,o.kt)("inlineCode",{parentName:"p"},"Sign and Submit")," transaction.",(0,o.kt)("img",{src:a(23534).Z,width:"1788",height:"876"})))),(0,o.kt)("p",null,"The Polkadot UI displays information about the content of the smart contract."),(0,o.kt)("p",null,"Depending on the account you used, you might be prompted for the account password. If you used a predefined account, you won\u2019t need to provide a password."),(0,o.kt)("h3",{id:"call-the-smart-contract"},"Call the smart contract"),(0,o.kt)("p",null,"Now that your contract has been deployed on the blockchain, you can interact with it. Our deployed smart contract has  functions \u2014 ",(0,o.kt)("inlineCode",{parentName:"p"},"totalSupply()")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"balanceOf()")," \u2014 and you can use the Polkadot UI to try them out."),(0,o.kt)("p",null,"To test the ",(0,o.kt)("inlineCode",{parentName:"p"},"balanceOf()")," function:"),(0,o.kt)("p",null,"Select any account from the Account list."),(0,o.kt)("p",null,"This contract doesn\u2019t place restrictions on who is allowed to send the ",(0,o.kt)("inlineCode",{parentName:"p"},"balanceOf()")," request."),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Click ",(0,o.kt)("inlineCode",{parentName:"li"},"Read"),". Verify that the value ",(0,o.kt)("inlineCode",{parentName:"li"},"1,000,000,000,000,000")," is returned in the Call Results.")),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(40420).Z,width:"1788",height:"876"})),(0,o.kt)("h3",{id:"rococo-faucet"},"Rococo Faucet"),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Canvas")," - parachain on ",(0,o.kt)("strong",{parentName:"p"},"Rococo")," \u2012 a testnet for ",(0,o.kt)("strong",{parentName:"p"},"Polkadot and Kusama parachains"),".\nAs a first step, you should create an account. ",(0,o.kt)("a",{parentName:"p",href:"https://wiki.polkadot.network/docs/learn-account-generation"},"See here for a detailed guide.")),(0,o.kt)("p",null,"As a second step, you have to get ",(0,o.kt)("inlineCode",{parentName:"p"},"ROC")," testnet tokens through the ",(0,o.kt)("a",{parentName:"p",href:"https://wiki.polkadot.network/docs/learn-DOT#getting-rococo-tokens"},"Rococo Faucet"),". This is a chat room in which you need to write:"),(0,o.kt)("p",null,(0,o.kt)("inlineCode",{parentName:"p"},"!drip YOUR_SS_58_ADDRESS:1002")),(0,o.kt)("p",null,"send message to ",(0,o.kt)("a",{parentName:"p",href:"https://matrix.to/#/#rococo-faucet:matrix.org"},"#rococo-faucet:matrix.org")),(0,o.kt)("p",null,"The number ",(0,o.kt)("inlineCode",{parentName:"p"},"1002")," is the parachain id of ",(0,o.kt)("strong",{parentName:"p"},"Canvas on Rococo"),", by supplying it the faucet will teleport ",(0,o.kt)("inlineCode",{parentName:"p"},"ROC")," tokens directly to your account on the parachain"),(0,o.kt)("h3",{id:"deploy-to-canvas"},"Deploy to Canvas"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Navigate to the ",(0,o.kt)("a",{parentName:"li",href:"https://polkadot.js.org/appshttps://paritytech.github.io/contracts-u"},"Polkadot.js.org")," in a web browser."),(0,o.kt)("li",{parentName:"ul"},"Verify that you are connected to the ",(0,o.kt)("strong",{parentName:"li"},"Contracts Node"),".")),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(49048).Z,width:"1788",height:"876"})),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Upload ",(0,o.kt)("inlineCode",{parentName:"li"},"my_psp22.contract")," file the same way as to local node but we need some ",(0,o.kt)("inlineCode",{parentName:"li"},"ROC")," tokens"),(0,o.kt)("li",{parentName:"ul"},"Use wallet which contains ",(0,o.kt)("inlineCode",{parentName:"li"},"ROC")," tokens")),(0,o.kt)("h3",{id:"astar"},"Astar"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},"Astar")," - Astar is a multi-chain smart contract platform that supports multiple\nblockchains and virtual machines."),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},"Astar/Shiden Network Family:"),"\nBefore starting the deployment, it's important to understand Astar/Shiden Network family. You should change the network based on what you want to do. Currently, there are 3 networks available, ",(0,o.kt)("strong",{parentName:"li"},"Shiden"),", ",(0,o.kt)("strong",{parentName:"li"},"Shibuya"),", and ",(0,o.kt)("strong",{parentName:"li"},"Local")," network. All networks support own standard Substrate RPC and EVM RPC."),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},"Astar and Shiden"),":\nAstar is the network that aims to be the parachain of Polkadot. Shiden is the sister network of Astar which is the parachain of Kusama. Basically, Astar and Shiden share the same code base. The biggest difference is the economic impact.")),(0,o.kt)("p",null,"Please note that Shiden has its real economic value. So you need to pay in SDN, the native token of Shiden, when you execute transactions. You can buy SDN on crypto exchanges."),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},"Shibuya"),":\nShibuya is the test network of Shiden and is connected to our own Relaychain. So Shibuya behaves almost the same as Shiden. Any new features are tested on Shibuya first and then deployed on Shiden. SBY, the native token of Shibuya, has no economic value and is available through our ",(0,o.kt)("a",{parentName:"li",href:"https://docs.astar.network/integration/testnet-faucet"},"faucet"),". The best practice is to testing smart contract on Shibuya before deploying it on Shiden to check whether your smart contract works well or not."),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},"Astar local Network"),":\nHere is ",(0,o.kt)("a",{parentName:"li",href:"https://docs.astar.network/tutorial/develop-and-deploy-your-first-smart-contract-on-aster-shiden-evm/running-local-network"},"tutorial")," how to run local network")),(0,o.kt)("h3",{id:"deploy-to-shibuya"},"Deploy to Shibuya"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Build smart contract the same way as for ",(0,o.kt)("a",{parentName:"li",href:"#build"},"local node")),(0,o.kt)("li",{parentName:"ul"},"Be sure that you have polkadot ",(0,o.kt)("a",{parentName:"li",href:"https://docs.astar.network/stake2earn-festival/how-to-make-a-kusama-polkadot-address#recommend-polkadot-.js-browser-plugin"},"wallet")," exension in your browser"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://docs.astar.network/tutorial/how-to/how-to-make-a-kusama-polkadot-address#create-account"},"Create polkadot account")," if not have yet"),(0,o.kt)("li",{parentName:"ul"},"Use ",(0,o.kt)("strong",{parentName:"li"},"Faucet")," to get free ",(0,o.kt)("strong",{parentName:"li"},"SBY")," ",(0,o.kt)("a",{parentName:"li",href:"https://docs.astar.network/integration/testnet-faucet"},"token")),(0,o.kt)("li",{parentName:"ul"},"Go to ",(0,o.kt)("a",{parentName:"li",href:"https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.shibuya.astar.network#/explorer"},"polkadot.js.org")),(0,o.kt)("li",{parentName:"ul"},"Switch network to ",(0,o.kt)("strong",{parentName:"li"},"Shibuya")," and deploy contract")),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(58953).Z,width:"1788",height:"876"})),(0,o.kt)("p",null,"We use ",(0,o.kt)("strong",{parentName:"p"},"\u201cmessages\u201d")," to communicate with smart contracts."),(0,o.kt)("p",null,"There are 2 types of messages:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"messages that change a smart contract\u2019s state should be sent as transactions"),(0,o.kt)("li",{parentName:"ul"},"messages that don\u2019t change a state can be made by using RPC calls")),(0,o.kt)("p",null,"Next, let\u2019s change the smart contract state by sending a transaction that calls the ",(0,o.kt)("inlineCode",{parentName:"p"},"transfer()")," function."),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(54830).Z,width:"1786",height:"876"})),(0,o.kt)("p",null,"As expected, the value that was stored in the smart contract changed from ",(0,o.kt)("inlineCode",{parentName:"p"},"0")," to ",(0,o.kt)("inlineCode",{parentName:"p"},"1")," after the ",(0,o.kt)("inlineCode",{parentName:"p"},"transfer()")," transaction is successfully executed"),(0,o.kt)("p",null,(0,o.kt)("img",{src:a(52032).Z,width:"1786",height:"876"})),(0,o.kt)("p",null,"Congratulations, you deployed and test your first L1 Smart Contract to ",(0,o.kt)("strong",{parentName:"p"},"Shibuya")," network!"))}d.isMDXComponent=!0},98687:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220604_183027_go-to-polkadot-4fcf7d8acc20c0aa8432bd87bede1290.gif"},23534:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_122254_upload-contract-f3213c196501cf06ef9a8e6721dabd29.gif"},40420:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_124705_balance-of-159f9134bd2c8692a672cd60ef14dc3d.gif"},49048:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_125943_contracts-node-fc7343c5bb005f9b261c51e3ee68a762.gif"},58953:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_132655_shibuya_testnet-9d361ecde01ab79b814c8c9d9b267b44.gif"},54830:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_132803_transfer-shibuya-17a6b0bc7d61d23f7fefea9d7c08ee0f.gif"},52032:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_133034_check-balance-of-shibuya-22d01db1423e1b22af6677be4eb07d15.gif"},80907:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/20220605_155001_create-wallet-8121bac17c915c543c0d7986024644d3.gif"}}]);