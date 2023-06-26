complete -c csview -s d -l delimiter -d 'Specify the field delimiter' -r
complete -c csview -s s -l style -d 'Specify the border style' -r -f -a "{none	,ascii	,ascii2	,sharp	,rounded	,reinforced	,markdown	,grid	}"
complete -c csview -s p -l padding -d 'Specify padding for table cell' -r
complete -c csview -s i -l indent -d 'Specify global indent for table' -r
complete -c csview -l sniff -d 'Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit' -r
complete -c csview -l header-align -d 'Specify the alignment of the table header' -r -f -a "{left	,center	,right	}"
complete -c csview -l body-align -d 'Specify the alignment of the table body' -r -f -a "{left	,center	,right	}"
complete -c csview -s H -l no-headers -d 'Specify that the input has no header row'
complete -c csview -s n -l number -d 'Prepend a column of line numbers to the table'
complete -c csview -s t -l tsv -d 'Use \'\\t\' as delimiter for tsv'
complete -c csview -s h -l help -d 'Print help'
complete -c csview -s V -l version -d 'Print version'
