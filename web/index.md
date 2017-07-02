## About
Howdy, my name is Lukáš Hozda (unless you already noticed that \*cough cough\*)
and I come from Czech Republic. I happen to like programming, software development,
C family languages, and well, programming languages and their design in general.
I can also do some web development, but it is not among my main fields of interest.
What technologies I use and what can I do is listed down below

The purpose of this website is to hold basic information
about me, some of my notable projects, my rants, some of my knowledge
and to work as a webserver paired with __Pebble__, a package manager for
the C2 language.

You might also take a moment to read the
story about my path in programming, seen in the Articles section,
or take a moment to appreaciate the worst monstrosity I ever wrote in C
as code which wasn't intentionally obfuscated.

```
	for(int32 i = 0; i < tree->children_num; i++)
  {
    if(strcmp(tree->children[i]->tag, "target|>") == 0
      && (searchstr(wanted, "all") 
        || searchstr(
            wanted,
            tree->children[i]
            ->children[1]
            ->contents
          )
        )
      )
        mpc_ast_print(tree->children[i]);
    if(strcmp(tree->children[i]->tag, "system|>") == 0)
        for(int32 j = 0; j < tree->children_num; j++)
            if(strcmp(
            	tree->children[i]
            	->children[j]
            	->tag,
            	"target|>"
            ) == 0
              && (searchstr(wanted, "all") 
                || searchstr(
                    wanted,
                    tree->children[i]
                    ->children[j]
                    ->children[1]
                    ->contents
                  )
                )
              )
                mpc_ast_print(tree->children[i]->children[j]);
	}
```

Thanks for appreciating it. Really stops looking like C, doesn't it?
As a reward, I get to tell you that all the important websites
are on the left side. So far, I got __Pebbles__, which contains 
a automatically generated index of packages (aka pebbles) of the 
Pebble manager; __Home__ for this page, __Style__, which
describes the cutting edge Allsucks coding style; __Articles__, which
contains, well, articles; __Projects__, which lists all my notable
projects and finally __Socks/Rocks__, which contains rants and praises,
lists of things that are cool and things that are not
and code snippets.

There is also a few hidden pages (read failures). Try to find them!
Or maybe don't.

## Programming languages
My favorite programming languages, the ones I will never forget:
* __C#__
* __C__
* __Rust__
* __C2__

The other ones I used a bit more. However, from the latter two, I can't say
I remember that much. They are past their primes anyways. It was good while it lasted.
Except VB. VB was agony.
* __Javascript__
* __Actionscript 3__
* __Visual Basic__

The ones I know something something and would be willing to learn more, maybe.
Especially the functional programming is tempting when I look at F#. With Ceylon,
I hope that the folks at RedHat finally made a JVM language that doesn't suck:
* __F#__
* __C++/CLI__
* __C++__
* __Go__
* __OOC__
* __Ceylon__

The ones of my childhood, which have almost faded from my memory. Only good memories
remain. I really recommend Small Basic and NetLogo as entry languages for kids. Especially
the NetLogo turtles are lovely:
* __Small Basic__
* __Turbo Pascal__
* __Basic__
* __NetLogo__

The ones I'd rather forget and am doing a pretty good job at that, to be honest. These
ones are concentrated agony, I wouldn't wish upon anyone. Java is a tricky one,
because if you are learning it as the first language, you won't notice it sucks balls
until too late or until you are already brainwashed by it:
* __VB.NET__
* __Scratch__
* __Google Blockly__
* __Java__

The ones I think are not bad, but which I didn't give time. Might learn, but won't
be among my favorite ones for one reason or another. So far I like newsqueak the most,
it's a bit on the obsolete side, though:
* __Python__
* __Ruby__
* __Perl__
* __Newsqueak__

### Other computer languages
* __HTML__
* __CSS__
* __Jade aka Pug__
* __TOML__
* __JSON__ (does it even count?)
* __Rusty__ (I invented that one, so the same question?)

### Technologies
* __UE4__
* __Linux__
* __Windows__
* __Office applications__
* __MS Paint__
* __GDB debugger__ ♥
* __Construct__
* __Make__
* __CMake__ (just a tad)

### Editors
* __Visual Studio__
* __Notepad__ ♥
* __Sublime Text 3__
* __Micro__ (a very modern terminal editor)
* __Nano__ (the trustiest of compeers)
* __Sandy__ (fast and tiny in terminal)
* __ed__ ___ED IS THE STANDARD EDITOR___

### Frameworks/Favourite libraries
* __JQuery__
* __mpc__
* __ZedNet__
* __libc__ ♥
* __clap-rs__
* __Hyper__
* __Pencil__ (SharpPencil fork)
* __Suckless 'arg.h'__
* __LiteDB__

### Things I wouldn't even touch with a stick
* __Java__
* __COBOL__
* __macOS__
* __iOS__
* __Anything Apple, really__
* __CS:GO__ (the people, not the game itself)
* __LOL__ (ditto)
* __DOTA__ (also ditto)
* __People who think programmer doubles as IT technician,
  "Hollywood hacker", sysadmin, mathematician, nerd and plumber__

## Contacts
* Email: luk.hozda@gmail.com
* Skype: luciusmagn
* Github: luciusmagn
* IRC (#rust-beginners at irc.mozilla.org): magnusi
* Steam: luciusmagn
* Discord: luciusmagn
* Display name in vast majority of places: magnusi

*Man, I am creative with names, am I not?*

## Contribution
Contributing to this page is permitted, just clone
the repo, make your changes and create a pull-request.
Tada. However, I am the person who knows most about
me, so I don't know what might one want to add about
me here. If you want to add a juicy rant or why Java
sucks, I am 100% merging. Let the hate flow through.
__[insert Palpatine picture]__
