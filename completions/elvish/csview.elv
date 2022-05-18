
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
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -H 'Specify that the input has no header row'
            cand --no-headers 'Specify that the input has no header row'
            cand -n 'Prepend a column of line numbers to the table'
            cand --number 'Prepend a column of line numbers to the table'
            cand -t 'Use ''\t'' as delimiter for tsv'
            cand --tsv 'Use ''\t'' as delimiter for tsv'
        }
    ]
    $completions[$command]
}
