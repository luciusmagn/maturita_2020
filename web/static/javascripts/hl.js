$(document).ready(function() {
	hljs.configure({
		tabReplace: '   ',
	})

	$('code').each(function(i, block) {
        hljs.highlightBlock(block);
    });

    $('code').addClass('hljs');
})
