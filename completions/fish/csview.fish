complete -c csview -s d -l delimiter -d 'Specify the field delimiter' -r
complete -c csview -s s -l style -d 'Specify the border style' -r -f -a "{None	,Ascii	,Ascii2	,Sharp	,Rounded	,Reinforced	,Markdown	,Grid	}"
complete -c csview -s p -l padding -d 'Specify padding for table cell' -r
complete -c csview -s i -l indent -d 'Specify global indent for table' -r
complete -c csview -l sniff -d 'Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit' -r
complete -c csview -s h -l help -d 'Print help information'
complete -c csview -s V -l version -d 'Print version information'
complete -c csview -s H -l no-headers -d 'Specify that the input has no header row'
complete -c csview -s n -l number -d 'Prepend a column of line numbers to the table'
complete -c csview -s t -l tsv -d 'Use \'\\t\' as delimiter for tsv'
