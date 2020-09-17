
edit:completion:arg-completer[csview] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'csview'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'csview'= {
            cand -d 'Field delimiter'
            cand --delimiter 'Field delimiter'
            cand --style 'Border style'
            cand -H 'Set if csv has no title'
            cand --no-headers 'Set if csv has no title'
            cand -t 'Use ''\t'' as delimiter for tsv, overrides ''-d'' option'
            cand --tsv 'Use ''\t'' as delimiter for tsv, overrides ''-d'' option'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand completion 'Generate shell completion file'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'csview;completion'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'csview;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
