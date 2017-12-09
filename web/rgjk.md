<img src="https://www.rust-lang.org/logos/rust-logo-256x256-blk.png" style="display: block;margin: 0 auto;"></img>
<h1 style="text-align: center">GJK Rust - Kompletní návod</h1>

Kompletní návod pro programování v Rustu podle letošního
předmětu. S postupem času se bude návod prodlužovat. Kroky
jsou doporučeným řešením, v mnoha případech jsou i alternativní
způsoby. Nemám zkušenosti s OSX, takže pokud je něco špatně, napište
mi: __hozda@gjk.cz__. Návod obsahuje příkazy. Na Linuxu/OSX pro
terminál, na Windows pro příkazovou řádku/CMD.exe.

## Obsah
- [Info](#info)
- [Příprava](#priprava)
- [Úvod](#uvod)
- [Aritmetika a Proměnné](#aritmetika)

<h2 id="info">Info</h2>  


Na Windows se příkazová řádka otevře buď přes nabídku Start, nebo
v jakékoliv složce pomocí zkratky Shift+Right-Click, která přidá
možnost otevření příkazové řádky v dané složce do kontextové nabídky:

<img src="https://i.stack.imgur.com/0DLsh.png"></img>

Na Linuxu se terminál otevírá buď přes kontextovou nabídku ve
složce, přes menu (jako na Windows), nebo pomocí zkratky Alt+F2:

<img src="http://cdn.makeuseof.com/wp-content/uploads/2011/01/altf2-main.png?x97327"></img>

V některých případech návod vyžaduje, aby byla příkazová řádka v
určité složce. K tomu lze využít několik jednoduchých příkazů. Znaky za #
jsou komentáře, to se do příkazového řádku nepíše:

Návod předpokládá, že se čtenář alespoň zběžně podíval na prezentace
z hodin na <https://github.com/rust-gjk/Materialy>


### Windows
```bash
cd         #přesune se do domovské složky
cd ..      #přesune se do nadřazené složky - 'projekt\src\' -> 'projekt\'
cd projekt #přesune se do složky 'projekt', nacházející se ve stávájícím adresáři
cd C:\programovaní\projekt #přesune se do složky

dir #zobrazí seznam souborů ve stávající složce
echo %cd% #zobrazí název stávající složky (resp. cesty)
```

### Linux
```bash
cd         #přesune se do domovské složky
cd ..      #přesune se do nadřazené složky - 'projekt/src' -> 'projekt/'
cd projekt #přesune se do složky 'projekt', nacházející se ve stávájícím adresáři
cd /programovaní/projekt #přesune se do složky

ls #zobrazí seznam souborů ve stávající složce
pwd #zobrazí název stávající složky (resp. cesty)
```

POZN: Pokud nepoužívate vlastní počítač, doporučuji používat pořád ten samý erární počítač.
Pokud se bojíte o bezpečnost vašeho kódu, můžete vždycky repozitáře smazat a znovu
klonovat z Githubu (=stahovat) pomocí `git clone URL`, kde *URL* je odkaz na daný repozitář.

<h2 id="priprava">Příprava</h2>  

0. __Instalace Linuxu__ - tento krok je dobrovolný, Rust je totiž multiplatformní:  
	1. Vyberte si distribuci - pro začátečníky je vhodná nějaká variace Ubuntu
	   nebo případně Mint, Manjaro, Elementary OS, Solus a Linux Lite
	0. Na stránkách jednotlivých distribucí je návod instalace  
	0. Distribuce by měla mít tyto čtyři programy:  
		1. <u>Terminál</u>  
			- zpravidla už nainstalovaný
			- všechny příkazy jsou pro terminál
		0. <u>curl</u>  
			- často bývá nainstalovaný, pokud ne, otevřete terminál a napište
			  následující příkaz (podle distribuce):  
			  1. Ubuntu atd. - `sudo apt-get install curl`
			  0. Archové distribuce (Manjaro) - `sudo pacman -S curl`
			  0. Solus - `sudo eopkg install curl`
			- přítomnost programu se dá ověřit příkazem `curl --version`, který vypíše verzi curlu
				a skončí
		0. <u>Prohlížeč</u>
			- taky většinou nainstalovaný
				1. Ubuntu atd. - `sudo apt-get install firefox`
				0. Archové distribuce (Manjaro) - `sudo pacman -S firefox`
				0. Solus - `sudo eopkg install firefox`
		0. <u>Git</u>
			- může být nainstalovaný, ale z těchto programů je na to nejmenší šance
			- systém pro verzování, termín vysvětlen v prezentaci z druhé hodiny
				1. Ubuntu atd. - `sudo apt-get install git`
				0. Archové distribuce (Manjaro) - `sudo pacman -S git`
				0. Solus - `sudo eopkg install git`
			- přítomnost programu se dá ověřit příkazem `git --version`, který vypíše verzi gitu
				a skončí
			- po instalaci bude potřeba minimální konfigurace:
				- `git config --global user.email vas_email_na_github@domena.com`
				- `git config --global user.name "Vaše Jméno"`
0. __Instalace Rustu__  
	1. Nejdříve je potřeba nainstalovat Rustový toolchain (=sada nástrojů pro vývoj).
		 V případě Rustu to je překladač (__rustc__), balíčkovač (__Cargo__),
		 debuggery (__rust-lldb__ & __rust-gdb__)  a dokumentační nástroj (__rustdoc__)  
			&emsp;&emsp;1. OSX/Linux - `curl https://sh.rustup.rs -sSf | sh`  
			&emsp;&emsp;2. Windows - instrukce na <https://www.rust-lang.org/en-US/install.html> <br><br>
	0. Za chvíli se v terminálu, příkazovém řádku objeví toto:  
		 <img src="https://fungos.github.io/images/1/rustup-console.png"></img>
		 Pro předmět potřebujeme __nightly__ kanál, tzn:  
		 	&emsp;&emsp;1. Zmáčkněte klávesu __2__ a potvrdíte  
		 	&emsp;&emsp;2. Klávesu __Enter__ pro zachování tripletu  
		 	&emsp;&emsp;3. Napíšete __nightly__ jako kanál a potvrdíte  
		 	&emsp;&emsp;4. Napište __'y'__ pro potvrzení změny proměnné __$PATH__ <br><br>
	0. V rámci minut by se měl Rust stáhnout a nainstalovat
	0. Poté je potřeba restartovat počítač  
		- Na Linuxu/OSX je též možné použít příkaz `source $HOME/.cargo/env`, který umožní
			používání nástrojů Rustu okamžitě v daném terminálovém okně. Rust bude globálně
			použitelný až po restartování
0. __Konfigurace Rustu__ - Po instalaci je potřeba jeden příkaz na konfiguraci
	1. Windows/Linux/OSX - `rustup default nightly`
	0. Tento příkaz zvolí __nightly__ kanál jako výchozí
	0. Pro výuku používáme kanál __nightly__, protože je pro něj dostupných několik
		 užitečných nástrojů a lze používat experimentální části Rustu
0. __Instalace editoru__
	- Rust patří mezi jazyky, které nejsou přímo vázené na IDE
	(=integrované vývojové prostředí), takže programy může člověk vyvíjet s čímkoliv,
	třeba i Poznámkovým blokem. Hodí se ale nainstalovat si editor, který nám práci
	usnadní. V rámci předmětu silně doporučuji VSCode, není ovšem problém vybrat
	si i jiný editor.
	- Instalace VSCode
		- Windows: <https://code.visualstudio.com/Download>
		- OSX: <https://code.visualstudio.com/Download>
		- Linux:
			- Ubuntu: <https://code.visualstudio.com/Download>
			- Arch: `yaourt -S visual-studio-code-oss`  (pozorně čtěte instrukce na obrazovce)
	- Dále je také potřeba nainstalovat si rozšíření pro správnou podporu Rustu
		1. Otevřete paletu příkazů pomocí __Ctrl+Shift+P__
		2. Napište/Zvolte __Install Extensions__
		3. Objeví se boční panel, do něj napište _'Rust'_ a nechte instalovat první výsledek
		   vyhledávání (autor __kalitaalexey__)
		4. Restartujte VSCode
		5. Při prvním otevření nějakého Rustového zdrojového souboru se pravděpodobně bude
			 VSCode vztekat že nemá všechny nástroje a nabídne jejich instalaci, vše potvrďte
		6. Když se zeptá na výchozí toolchain, zvolte nightly
0. __Instalace Gitu__
	1. Pokud máte Linux, instrukce jsou u kroku 0 - instalace Linuxu
	0. Windows - <https://git-scm.com/download/win>
	0. OSX - <https://git-scm.com/download/mac>
	0. Po instalaci je zapotřebí provést minimální konfiguraci (příkazy opět do příkazového řádku/
		 terminálu)
		 - `git config --global user.email vas_email_na_github@domena.com`
		 - `git config --global user.name "Vaše Jméno"`

<h2 id="uvod">Úvod</h2>  

1. __Tvorba prvního programu__ - Hello World
	1. Hello World je malý počítačový program, který vypíše text "Hello World" do standardního
		 výstupu (=příkazová řádka nebo panel v editoru). První verze - v jazyce C - vznikla už
		 skoro před 40 lety.
	0. Někde v systému (pokud možno na dobře dostupném místě) si vytvořte složku, kde budete schraňovat
		 kód z hodin, případně domácí úkoly
	0. Otevřete příkazovou řádku/terminál a vstupte do složky (buď termininál otevřete přímo ve složce
		 nebo pomocí `cd`, `ls`/`dir`)
	0. Když budete ve složce, vytvořte nový projekt pomocí nástroje Cargo - `cargo new hello_world --bin`
		 - nezapoměňte na přepínač `--bin`, jinak Cargo vygeneruje knihovnu
	0. Pokud jste správně postupovali, bude to v adresáři s projekty vypadat zhruba takto:
	```bash
	.
	└── hello_world
	    ├── Cargo.toml
	    └── src
	        └── main.rs
	```
	6. Složku hello_world otevřte v editoru. Ve VSCode je možnost otevřít složku v nabídce Soubor, nebo
		 pomocí klávesové zkratky __Ctrl+K,Ctrl+O__
	0. V levém panelu se zobrazí soubory, otevřte __main.rs__ a uvidíte vygenerovaný Hello World
	```rust
	fn main() {
		println!("Hello, World!");
	}
	```
	8. Program lze spustit pomocí možnosti __Cargo: Run__ ve VSCode v paletě příkazů (Ctrl+Shift+P) nebo
		 příkazem `cargo run` v příkazovém řádku (musíte vstoupit do složky *hello\_world* pomocí `cd`)
	0. Co jednotlivé části programu znamenají je vysvětlěno v prezentaci; zkuste změnit text
0. __Uložení v repozitáři na Githubu__
	1. Tento krok je (zatím) též dobrovolný, slouží jako předzvěst správné správy kódu pomocí Gitu
	0. Cargo dělá ze svých projektů Gitové repozitáře automaticky, tak o to se starat nemusíme
	0. Na Githubu kliknete na znaménko __+__ v pravém horním rohu obrazovky nebo na tlačítko New Repository:  
		 <img src="http://www.softpost.org/wp-content/uploads/2016/06/new-repository-on-GitHub.png"></img>
	0. Vytvoříte nový repozitář. Čistě teoreticky, jméno je jedno ale pro konzistenci se hodí ho pojmenovat
		 __hello\_world__ jako projekt samotný. Nezaškrtávejte možnost "Initialize this repository with README":  
		 <img src="/static/repo-create-name.png"></img>
	0. Vstoupíte do složky *hello\_world* v příkazovém řádku/terminálu
	0. Přidejte repozitář na Githubu jako origin příkazem `git remote add origin URL` kde *URL* bude pravděpodobně
		 *https://github.com/__vase\_jmeno__/hello_world*. Zkrátka odkaz na váš repozitář na Githubu.
	0. Zaznamenáte všechny změny pomocí `git add .` (nezapoměňte na tečku a správné mezery)
	0. Vytvoříte revizi pomocí `git commit -m "první revize"` (pozor na uvozovky)
	0. Nahrajte změny na Github příkazem `git push -u origin master`
		 - část `-u origin master` se píše jen při prvním nahrávání změn
		 - pro další změny opakujte poslední 3 kroky

<h2 id="aritmetika">Aritmetika a proměnné</h2>  

1. __Kalkulačka v1.0__ - Variace na <strike>renesační téma</strike> Hello World
	1. Přesuňte se do složky, kde si schraňujete kód z hodin v příkazovém řádku/terminálu
	0. Vytvořte nový projekt pomocí `cargo new calc --bin` (nezapomeňte na `--bin`)
	0. Složku __calc__ ve vaší složce projektů otevřete ve VSCode pomocí __Ctrl+K,Ctrl+O__
	   a vložte následující kód:
	   ```rust
	   fn main() {
	       println!("mezi +nekonečno a -nekonečno");
	   }
	   ```
	4. Program otestujte pomocí `cargo run -- 4 + 5 * 2 ^ 2 / 2 - 9 << 1 >> 2 + 6`
		 - za dvě pomlčky napište libovolný výraz
		 - dvě pomlčky slouží jako oddělovač:  
		 	 - všechno před pomlčkami jsou parametry pro Cargo
		 	 - všechno za pomlčkami jsou parametry pro váš program
	0. Jestli vám program vypíše výsledek a ne chybu, tak jste právě stali autory
		 nejchytřejší kalkulačky na světě - kalkulačky, která dokáže vyhodnotit každý výraz,
		 včetně chybných a neúplných výrazů - kvantová extrapolace kalkulačky výrazy domyslí
	0. Bohužel výsledky, byť správné, nemají moc velkou přesnost. Vylepšíme je proto v další
		 iteraci kalkulačky
0. __Správa kódu__ - Postupně se vyvíjející programy jako je naše kalkulačka si přímo říkají
	 o používání nekterého verzovacího systému.  V našem případě to je Git v konjunkci s Githubem
	1. Cargo automaticky udělalo ze složky __calc__ repozitář, přesuňte se do ní v příkazovém řádku
	0. Vytvořte stejnojmenný repozitář na githubu
	0. Na Githubu kliknete na znaménko __+__ v pravém horním rohu obrazovky nebo na tlačítko New Repository:  
		 <img src="http://www.softpost.org/wp-content/uploads/2016/06/new-repository-on-GitHub.png"></img>
	0. Vytvoříte nový repozitář. Čistě teoreticky, jméno je jedno ale pro konzistenci se hodí ho pojmenovat
		 taky __calc__. Nezaškrtávejte možnost "Initialize this repository with README"
	0. Přidejte origin z Githubu pomocí `git remote add origin URL`, kde *URL* je odkaz na repozitář _calc_
		 - Pokud uděláte překlep, můžete origin smazat pomocí `git remote remove origin` a opakovat minulý příkaz
	0. Zaznamejte všechny změny pomocí `git add .` a vytvořte první revizi pomocí `git commit -m "první revize"`
	0. Nahrajte změny na Github příkazem `git push -u origin master`
		 - část `-u origin master` se píše jen poprvé
0. __Kalkulačka v2.0__
	1. Pokud nemáte otevřený editor a příkazový řádek/terminál, otevřete si znovu složku __calc__ v obojím
	0. V souboru main.rs nyní uděláme drobný upgrade - starý kód nahradíme tímto:  
		 ```rust
		 fn main() {
		     let a = 5;
		     let b = 2;
		     println!("vysledek: {}", a + b);
		 }
		 ```
	3. Spustíme pomocí __Cargo: Run__ z příkazové palety VSCode (__Ctrl+Shift+P__) nebo pomocí příkazu do
	   terminálu/příkazového řádku `cargo run`
	0. Kalkučka má nyní defacto 100% přesnost výsledků. Ovšem, za přesnost se platí. Moudrý muž kdysi řekl,
	   že programování je balancování kompromisů a hledání rovnováhy mezi radostí a frustrací.
	0. V prezentaci již je vysvětleno, co je proměnná (v programu jsou dvě - proměnná `a` a proměnná `b`).
	   Zkuste zaměnit výpočetní operaci za něco z `-, *, /, <<, >>, %` a měnit hodnoty proměnných.
	0. Zkuste přidat více proměnných a vypočítat něco trošku 'složitějšího', třeba plochu krychle nebo válce.
	0. Zkuste použít závorky ke změně pořadí vyhodnocování operací.
	0. Když budete spokojeni s výsledkem, vytvořte revizi pomocí `git add .` a `git commit -m "váš popis změn"`
	   a nahrajte změny na github s `git push`
0. __Kalkulačka v3.0__
	1. Pokud nemáte otevřený editor a příkazový řádek/terminál, otevřete si znovu složku __calc__ v obojím.
	0. main.rs prodělá další změnu:  
		 ```rust
		 fn main() {
		     let argumenty: Vec<_> = args().collect();
		     let a: i32 = argumenty[1].parse().unwrap();
		     let b: i32 = argumenty[2].parse().unwrap();
		     println!("vysledek: {}", a + b);
		 }
		 ```
		 Tentokrát bylo zavedeno několik pár věcí:  
		 - `let argumenty: Vec<_> = args().collect();` - vytvoří seznam sbírku argumentů příkazové řádky
		 	- _Vec<\_>_ znamená Vector obsahující prvky o typu, který identifikuje sám překladač.
		 		- Vektor je jedno rozměrná sbírka (=normální seznam)
		 	- `args().collect()` posbírá všechny argumenty příkazové řádky a udělá znich sbírku.
		 - `argumenty[1].parse().unwrap()` - 'Vem druhý prvek sbírky `argumenty` (`argumenty[1]`),
		 		pokus se jej zkonvertovat na požadovaný typ (`.parse()`) a získej výsledek konverze nebo ukonči program
		 		(`.unwrap()`) 
		 - `argumenty[2].parse().unwrap()` - 'Vem třetí prvek sbírky `argumenty`... atd.'
	0. Sbírky začínají počítat od nuly; je to z historických důvodů, kdy sbírky byly pouhým ukazatelem na první prvek
		 a další prvky se získavaly přičítáním k adrese ukazatele (adresa + 0 == první prvek, adresa + 1 == druhý prvek ...)
	0. Program spustíte příkazem `cargo run -- cislo1 cislo2`, čísla si zvolte libovolná, jediná podmínka je aby byla celá
	0. Zkuste opět zaměnit početní operace, číst více čísel nebo číselné typy
	0. Když budete spokojeni s výsledkem, vytvořte revizi pomocí `git add .` a `git commit -m "váš popis změn"`
	   a nahrajte změny na github s `git push`

<h2 id="if">If - větvení kódu</h2>

0. __Ukázka standardní knihovny v Rustu__ - nepovinný bonus na začátek?
	1. Standardní knihovna je soubor funkcí, datových typů, konstant zorganizovaných do
		 několika modulů, které poskytují jazyku základní funkcionalitu a schopnost komunikace se
		 systémem
	0. Knihovny se v Rustu (a mnoha jiných jazycích) musí manuálně importovat, ale část standardní je
		 importovaná automaticky (viz <https://doc.rust-lang.org/std/prelude/>)
	0. Obsah standardní knihovny se najde v [dokumentaci](https://doc.rust-lang.org/std/)
	0. Na otestování standardní knihovny si můžete vytvořit nový projekt pomocí `cargo new stdlib --bin`
		 a postupem analogickým s předchozím kódem (navigace, otevírání v editoru, kompilace atd.)
		 nebo bude stačit i otestovat kód na Playgroundu - <https://play.rust-lang.org/>
	0. Každopádně vložte například následující kód:  
		 ```rust
		 use std::i32::MAX; // maximální hodnota 32-bitového čísla
		 fn main() {
		     println!("maximální hodnota i32 je {}", MAX);
		 }
		 ```
	0. Jako obvykle, kód se spustí pomocí `cargo run` nebo palety příkazů v editorů (__Ctrl+Shift+P__, Cargo: Run)
	0. V příkazovém řádku (resp. výstupu v editoru) by se měla objevit maximální hodnota __i32__
0. __Kalkulačka v4.0__ - používání cizí knihovny `text_io`
	1. Složka projektu obsahuje soubor Cargo.toml, který končí textem `[dependencies]`
	0. Na další řádek se přidá následující text:
		 ```toml
		 text_io = "*"
		 ```
	0. To způsobí, že při další kompilaci Cargo automaticky knihovnu stáhne a učiní použitelnou v programu
	0. V main.rs je nejprve nutné knihovnu importovat:
		 ```rust
		 extern crate text_io;
		 ```
	0. To učiní knihovnu viditelnou v programu. `text_io` je ovšem knihovna, která exportuje i makra, takže
		 v tomto případě je nutné řádek ještě trochu změnit:
		 ```rust
		 #[macro_use] extern crate text_io;
		 ```
	0. `#[macro_use]` je atribut. Ty mohou být aplikovány na importace, struktury, funkce a bloky kódu. Do hloubky budou
		 probrány až u struktur.
	0. Nyní můžeme kalkulačku změnit tak, aby se na parametry ptala uživatele:
		 ```rust
		 #[macro_use] extern crate text_io;
		 fn main() {
		     println!("zadejte první číslo: ");
		     let a: i32 = read!();
		     println!("zadejte druhé číslo: ");
		     let b: i32 = read!();
		     println!("vysledek: {}", a + b);
		 }
		 ```
0. __If__ - podmínky
	1. Podmínky slouží k tzv. větvění kódu. Někdy je potřeba dělat něco jiného, když se lyší vstup nebo prostředí programu
	0. Základní konstrukcí pro větvení ve většině modernějších (cca 30 let a mladší) programovacích jazyků je __if__
	0. Syntaxe ifu vypadá takto:
		 ```rust
		 if podmínka {
		 	// kód
		 }
		 else if další_podmínka {
		 	// kód, když neplatí podmínka, ale další_podmínka platí
		 }
		 else {
		 	// kód, když nic neplatí a existence je lež
		 }
		 ```
	4. Jedině část if-podmínka-blok je povinná a else-if se může opakovat. Narozdíl od mnoha jiných programovacích jazyků
		 se závorky kolem podmínek nepíšou (budou brány jako součást výrazu, což neovlivní výsledek), ale složené závorky jsou povinné.
		 Důvod pro to je jednoduchý. V Rustu je spousta věcí výraz, mimo jiné i bloky kódu, pokud poslední příkaz/výraz v něm nekončí středníkem.
		 Je tedy možné zplodit i takovéto neporučované zvěrstvo:
		 ```rust
		 if {
			 let a = get_number();
			 a * 2
		 } == {
			 let b = get_another_number();
			 b
		 } {
			 println!("dvojnásobek a je roven b");
		 }
		 ```
	5. Kód by byl velmi zavádějící kdyby tady `println!()` nemusel být v bloku. Bez bloků v if-ech se může programátor většinou obejít. Třeba minulý přiklad
		 by se dal jenodušeji přepsat bez použití bloků:
		 ```rust
		 if (get_number() * 2) == (get_another_number()) {
		 	println!("dvojnásobek prvního čísla je roven druhému")
		 }
		 ```
	6. Pro podmínky se používají tzv. relační operátory - `==, !=, <, >, >=, <=` - viz prezentace. Jejich výsledky jsou vždy booleovské hodnoty, tj. pravda - `true`
		 nebo nepravda - `false`. Proto se `==` a `!=` nikdy nepužívají k porovnávání booleovských hodnot v podmínkách. `promenna == true` je tedy zbytečné a častá
		 začátečnická chyba.
	0. Všechny booleovské výrazy se dají znegovat pomocí vykřičníku -> `!true`, `!(a != b)`
0. __Kalkulačka v5.0__ - možné řešení
```rust
#[macro_use] extern crate text_io;

fn main() {
	let c1: f64 = read!();
	let c2: f64 = read!();
	let op: String = read!();

	if op == "+" {
		println!("{}", c1 + c2);
	}
	else if op == "-" {
		println!("{}", c1 - c2);
	}
	else if op == "*" {
		println!("{}", c1 * c2);
	}
	else if op == "/" {
		println!("{}", c1 / c2);
	}
	else {
		println!("neznámá operace");
	}
}
```
