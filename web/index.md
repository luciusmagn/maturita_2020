## About
This website is about me. I am a czech citizen,
who happens to like programming, software development, C family
languages and programming languages and their design in general.
I can also do some web development, but it is not my field of
interest. The purpose of this website is to hold basic information
about me, some of my notable projects, my rants, some of my knowledge
and to work as a webserver paired with __Pebble__, a package manager for
the C2 language.

If you lack any common sense, you might take a moment to read the
story about my path in programming, written down below, or take a moment
to appreaciate the worst monstrosity I even wrote in C.


	for(int32 i = 0; i < tree->children_num; i++)
	{
    if(strcmp(tree->children[i]->tag, "target|>") == 0
       && (searchstr(wanted, "all") || searchstr(wanted, tree->children[i]->children[1]->contents)))
        mpc_ast_print(tree->children[i]);
    if(strcmp(tree->children[i]->tag, "system|>") == 0)
        for(int32 j = 0; j < tree->children_num; j++)
            if(strcmp(tree->children[i]->children[j]->tag, "target|>") == 0
               && (searchstr(wanted, "all") || searchstr(wanted, tree->children[i]->children[j]->children[1]->contents)))
                mpc_ast_print(tree->children[i]->children[j]);
	}

Thanks for appreciating it. As a reward, I get to tell you that
all the important websites are on the left side. So far, we got
__Pebbles__ which contains a automatically generated index of 
packages (aka pebbles) of the __Pebble__ manager and Home for this
page. There is also a few hidden pages (read failures). Try to find them!
Or don't. 

## Programming languages
Over the years, I have played with many programming languages,
they fascinate me. I mostly settled with the C-family languages,
including __C__ itself, but I never limited myself to it.  
My beginnings were pretty rough, I started with these:
* __Small Basic__
* __Turbo Pascal__
* __Basic__
* __Javascript__

Needless to say, even __Javascript__ was a total failure, back then.
My understanding of general programming concepts was simply not
deep enough.  

__Small Basic__, a simple programming environment for
kids, was amazing though. I didn't write a single conditional
in my entire time with it, but I was quite happy with making
colorful command-line programs which printed somewhat static
text or programs which draw uneven circles filled with text
written using a crappy font, which wasn't centered.  

A special place in my heart has __Turbo Pascal 5.5__. It taught
me some concepts of __OOP__, which was detrimental for my advancement.
Getting the binaries was very hard to do, because __Turbo Pascal__ was
pretty old even then. Luckily, since the computer I used was
a 32-bit beast barely able to run Win 98, running the 16-bit binaries
was easy. The manual I have for it was a huge help for me, because
it's in __Czech__. Needless to say, it wasn't a complete manual, but just
the changes from last versions turned into a by-example guide.
I didn't know that then, though. Ah, where are the days, when
I wrote code on a typewriter.  

The aforementioned languages occured at ages 7-9. I started early,
heh. Then came a second batch. These ones are ages 9-12,
but my success in them was still severely limited. For these, I
had a proper documentation already (yay!).
* __VB.NET__
* __Actionscript 3__
* __NetLogo__

__Actionscript__ was a defining language for me. Thanks to it I learned
proper programming concepts, how does OOP work in practice and a wee
bit about video game development. I am really sad that Flash died,
only because of how __Actionscript 3__ was great for me. Many thanks to
__Shaymus22__ for his tutorials, which started out great, but he never
bothered to finish them. Otherwise, flash really deserves to die. Luckily,
we still have Haxe and Haxe sounds neat, although I didn't play with it much.  

I started with __VB.NET__ when I wanted to do some hacking (in the correct
meaning of the word, not the distored one made by media) on __Terraria__
and saw a splendid tutorial on Youtube. You know, these tutorials by
an awesome hackerman, who writes in a big decorated font in Notepad,
records in 360p and makes several programming murders a minute. I never
finished that tutorial, but at least I knew how __VB.NET__ looks. And I found
it weird. I guess it was mostly because of all the time I spent with __AS3__.
I later used it to develop simple, yet nice looking web browsers. Learning
how to embed Gecko instead of using whatever that crappy web engine
WinForms contain by default was a great breakthrough for me. Anyways,
the most important thing about __VB.NET__is that it introduced me to two
things: __Visual Studio__ and the __.NET__ platform.  

At the time, I also spent a few months playing with __NetLogo__. I loved
those distorted arrow pointers, which meant to represent __turtles__.
Fractals, make lag with 1000 __turtles__, more lag crash. It was spectacular.
I think it is a great language & environment to introduce kids to.  

Because of school, I had to have a rendezvous with three more technologies,
after those languages. I call them technologies, because I don't count
them as programming languages per se.
* __Scratch__
* __Google Blockly__
* __Construct__

Even now, I still think __Scratch__ is a piece of shit. Like why the fuck
would someone teach that to children. IO possibilites? Zero. Flexibility?
Zero. Do you have to manually reset program to start state after each run?
Yes. Can entities leave the boundaries of the scene? No. Does the code
execute in real-time? No. __What the fuck__.  

__Google Blockly __was a nice spin on blocks introduced in __Scratch__. Making
the screen flicker with random colors accompanied with random rectangles
was nice. Good job, Google.  

__Construct__  was what I used for the final Scratch project instead of using
__Scratch__. I find it much better and more suited for simple game development.
Especially when working with kids. Programs now actually act like programs.
It can also compile to __HTML5__&__JS__&__CSS__ trifecta, which is great if you want
kids to be able to display their games on their webpages.  

After this fun time came the age of __.NET__. It also signifies a third batch
of languages that I worked with. Successes with these ones were starting
to get pretty neat. Likey-likey. __.NET__ is lovely.
* __C#__
* __Visual Basic__
* __F#__
* __C++/CLI__

__Visual Basic__ was at the beginning, I admit that. I had to use it, because
I was putting my shitty web-browser developing skill to use for someone
else. Turns out communication with mongolian people isn't perfect. Or maybe
it was that I abandoned them because I found them boring, their search
engine shitty and ideas too bold. Memory's foggy.  

 Shortly after came __C#__, which has been for long (and still is) my main boat.
Whenever shit goes sideways in a programming language, I still have __C#__ to go
back to. It is a language with a near perfect learning curve, nice prefered
coding style (__I love Allman__), __LINQ__, which is in its own cathegory of amazing
and a great supply of libraries. Splendid integration with __Visual Studio__
makes it a very powerful tool with rapid development speed.  

__F#__ is interesting. It is the youngest entry in this batch. It is also the
only functional programming language in the list. It is a great language,
but the learning curve might be a bit steeper for someone. Same goes for any
other functional language, I guess. Glad Microsoft provides the option, though.  

__C++/CLI__ is an interesting critter. It is an amalgamation of unmanaged __C++__ and
the .NET platform. If you stay in the 'clear', that means without using unmanaged
__C++__, it is an intriguing alternative to __C#__. However, steeper learning curve,
a lot of new concepts and the fact that it's not really well known make it a
less favorable choice. If you want to obfuscate your __.NET__ without paying
__$500__ for that shitty new obfuscator which has ads everywhere, using both
managed and unmanaged features of __C++/CLI__ is the way to go. Disassemble that,
bitch.  

Now we are getting to the last batch of programming languages so far. These
ones are the dreaded, unmanaged beasts. An interesting combination of fresh
blood and old dogs, which can actually learn new tricks.
* __C__
* __C++__
* __Rust__
* __Go__
* __C2 ('tis a special one)__

I briefly scratched the surface of __C__, when I randomly tried it somewhere
between batch 1 and 2. I returned to __C__ after my time with __C2__ (which would
actually mean going backwards with our philosophy) and hell, am I glad that I
did. __C__ taught me a lot of valuable things about how things really work, what's
underneath and how to make programs faster. It was also the first language
that made me worry much more about keeping good style and organization, since
there was no Visual Studio to do it for me.  

So far, in my __C2__ escapades, I also scratched the __C++__ surface. __C++__ is a behemoth
of a language, which has undeniably many good things going for it, but also
many against it. It's like two armies colliding, both of the same country,
clothed in the same clothes and you don't know which one's winning, because there
is just so many of them. I sometimes feel like Bjarne Stroustrup is a jerk, but
he seems to be a cool guy.  

My master plan was to learn __C++__ to be able to contribute to the __C2C__ compiler in
a greater degree, but I dropped the language completely after I discovered __Rust__.
__Rust__ is an amazing language. The learning curve is very steep, but boy, the reward
for it is sure great. __Rust__ took many old concepts which didn't become popular and
combined & implemented them in a way that makes Rust dangerously rewarding. Patterns
are a godsend; Traits, impls and struct trifecta takes the best of __classy__ (pun-intended)
and __C__ worlds; built-in iterators which provide functionality similar to __LINQ__ are
awe-inspiring and don't forget the __Result__ and __Option__ types, which make Rust a language
of no nulls and rare exception, which can all be prevented and handled before they
occur. Not to mention borrow checker and extremely pedant compiler, which just won't
let you fuck up your memory. And if you are unsure if there is a smarter way
to do something, run it to through __clippy__, awesome linter which usually even tells
you how to fix it.  

I also briefly tried __Go__ and I was deeply disappointed with it. Good things first,
though. Web frameworks and all web-related libraries are splendid. The language design
is simple and easy to pick up. Compilation times are short. BUUUT... Garbage collector
is retarded and slows down the language significantly. And most importantly, __Go__ doesn't
let you use Allman style brackets. It will literally break your code and net you
some weird exceptions never afore seen by the likes of ye'. Also semicolons in __Go__...
they tried to make it so smart that it's not. On the plus-size (puns puns), if you get
assimilated, it might be a breeze, enjoyable language. You need to get assimilated,
though.  

And last, but certainly not least, comes the __C2__ language. __C2__ language is a small
language, which tries to be a evolution of __C__. I found it on Github one day and
fell in love with it instantly. When I joined, __C2__ was in somewhat early stages.
No libraries, not even anything to use as standard library. But it had concepts,
the concepts were great and were fullfilled. I contribute to the language very
often and lead discussions with __Bas van den Berg__, the creator. My __C2__ time teaches
me a lot about compilers and good language design, which __C2__, in its simplicity,
certainly has. The only thing I don't like is that symbol identifiers must follow
somewhat strict rules, unless you want an error. And it can't be disabled.  

And well, that's all for my languages. If you read all the way here, then I would
like to have as much free time to waste as you do. Like really, this is quite
the wall of text. Don't you have better things to do? Well, if not, I also wrote
a few lines about other languages I disgraced down below.

## Other languages
Oh yes, my other languages. Now the special thing about these 'other' languages
is that they are not strictly __programming__ languages. I'd say the unifying
term would be __computer__ languages, but you can be never sure, can you?
* __HTML__
* __CSS__
* __Jade aka Pug__
* __Java (ew)__
* __Python__
* __TOML__
* __UE4__
* __OOC__

Well, the first one on the list is __HTML__. I believe that despite all the oddities
about it, it is something every new wannabe developer should know. With __HTML5__,
there has been a lot of changes and new things introduced to the language,
but I believe that those are not critical in my opinion. That is, unless you
want to study web development. Then even those are a must, sorry!  

__CSS__ is a critter which is often neglected by developers. People tend tend to think
of it as an accessory to __HTML__ and __Javascript__, but __CSS__ is a dangerous tool of it's own.
And unlike __HTML__ & __JS__, shitty code in __CSS__ is very visible. It can't be stressed
enough that it is detrimental for future developers to master it.  

__Jade__ aka __Pug__ (due to some licensing issues with the name) is a templating language
for Node. It is nice if you want to rid yourself of __HTML__ tags everywhere (read as
trade them for indentation). I don't like their new logo, though. __2/10__.  

Well, I, a __sworn Java-hater__ have also touched __Java__ in the past. For starters,
I believe that bashing something without trying it first is foolish, but I also found
Minecraft mods a very tempting thing to develop. Sheesh. The problem is __Java__ is that
for many, it's the first and only language. People tend to protect their 'firsts' and
fail to see their flaws. __Java__ has many of them and I am sure to give them some nice
bashing on this website. Also there is marketing, which I believe is a strong
theme with __Java.__ Just look at how the promises of __Java 8__ worked out. How can you
believe in __Java 9__ after that?  

I did also try __Python__. __Python__ is a nice language for quick scripts. Quick as in
development speed, not execution speed, hell naw. __Python__ has a lot of nice idea
and is a great language overall. It didn't stick with me because I don't like
its syntax and being forced to indent my code in only one, dictated way.  

__TOML__ is a nice new format for storing data. Much easier to edit than __JSON__,
much simpler to implement than __YAML__, much better specification than __INI__. What else?  

I know that __Unreal Engine 4__ is a language, but an engine, yeah. I still felt like
I should include it here, because it is the only means of game development that
stuck with me for more than a months and continues to do so. Graphics are great,
Blueprints are fairly simple (although sometimes lacking in speed) and physics work
nice.  

And last in this list is __OOC__. __OOC__ is a cute language that compiles to __C99__ and pure
happiness. The people who wrote it are as insane as I am, and that's a great plus.
__OOC__ means unicorns and rainbows and simple __OOP__ with weird but __Allman-allowing__ syntax.

## Contact
* Email: luk.hozda@gmail.com
* Skype: luciusmagn
* Github: luciusmagn
* IRC (#rust-beginners at irc.mozilla.org): magnusi
* Steam: luciusmagn

## Contribution
Contributing to this page is permitted, just clone
the repo, make your changes and create a pull-request.
Tada. However, I am the person who knows most about
me, so I don't know what might one want to add about
me here. If you want to add a juicy rant or why Java
sucks, I am 100% merging. Let the hate flow through.
__[insert Palpatine picture]__
