
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'csview' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'csview'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'csview' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Specify the border style')
            [CompletionResult]::new('--style', 'style', [CompletionResultType]::ParameterName, 'Specify the border style')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Specify padding for table cell')
            [CompletionResult]::new('--padding', 'padding', [CompletionResultType]::ParameterName, 'Specify padding for table cell')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Specify global indent for table')
            [CompletionResult]::new('--indent', 'indent', [CompletionResultType]::ParameterName, 'Specify global indent for table')
            [CompletionResult]::new('--sniff', 'sniff', [CompletionResultType]::ParameterName, 'Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-H', 'H', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Prepend a column of line numbers to the table')
            [CompletionResult]::new('--number', 'number', [CompletionResultType]::ParameterName, 'Prepend a column of line numbers to the table')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv')
            [CompletionResult]::new('--tsv', 'tsv', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
