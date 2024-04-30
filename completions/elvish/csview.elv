
use builtin;
use str;

set edit:completion:arg-completer[csview] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'csview'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'csview'= {
            cand -d 'Specify the field delimiter'
            cand --delimiter 'Specify the field delimiter'
            cand -s 'Specify the border style'
            cand --style 'Specify the border style'
            cand -p 'Specify padding for table cell'
            cand --padding 'Specify padding for table cell'
            cand -i 'Specify global indent for table'
            cand --indent 'Specify global indent for table'
            cand --sniff 'Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit'
            cand --header-align 'Specify the alignment of the table header'
            cand --body-align 'Specify the alignment of the table body'
            cand -H 'Specify that the input has no header row'
            cand --no-headers 'Specify that the input has no header row'
            cand -n 'Prepend a column of line numbers to the table'
            cand --number 'Prepend a column of line numbers to the table'
            cand -t 'Use ''\t'' as delimiter for tsv'
            cand --tsv 'Use ''\t'' as delimiter for tsv'
            cand -P 'Disable pager'
            cand --disable-pager 'Disable pager'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
