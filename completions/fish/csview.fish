complete -c csview -n "__fish_use_subcommand" -s d -l delimiter -d 'Specify the field delimiter' -r
complete -c csview -n "__fish_use_subcommand" -l style -d 'Specify the border style' -r -f -a "{None	,Ascii	,Sharp	,Rounded	,Reinforced	,Markdown	,Grid	}"
complete -c csview -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c csview -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c csview -n "__fish_use_subcommand" -s H -l no-headers -d 'Specify that the input has no header row'
complete -c csview -n "__fish_use_subcommand" -s t -l tsv -d 'Use \'\\t\' as delimiter for tsv'
complete -c csview -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c csview -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
