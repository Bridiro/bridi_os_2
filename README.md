# bridi_os_2
 
I'm currently following a tutorial from [Philipp Oppermann](https://github.com/phil-opp) on building a simple OS/Kernel completely in Rust (except for the bootloader that is done automatically).

[Project on Github](https://github.com/phil-opp/blog_os)

[Blog to follow step-by-step](https://os.phil-opp.com/)


## THE ISSUE

I followed step by step everything (and everything worked) until the [Intruduction to Paging](https://os.phil-opp.com/paging-introduction/) section. When I reached the Page Fault it just gave me:

paniked at 'misaligned pointer dereference: address must be a multiple of 0x4 but is 0xdeadbeaf', src/main.rs:18:14

So I tryed to figure it out for hours, but, unable to find the answer I just gave up and copyed the whole project from the [Official Github branch](https://github.com/phil-opp/blog_os/tree/post-08). Even after doing this, the output was the same. 

### Added information

After paniking the kernel just continue the normal execution, printing a lot of dots on every Timer interrupt, and print the corrisponding pressed key of the keyboard.