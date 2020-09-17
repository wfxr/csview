
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
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'csview' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'Specify the field delimiter')
            [CompletionResult]::new('--style', 'style', [CompletionResultType]::ParameterName, 'Specify the border style')
            [CompletionResult]::new('-H', 'H', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv, overrides ''-d'' option')
            [CompletionResult]::new('--tsv', 'tsv', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv, overrides ''-d'' option')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion file')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'csview;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'csview;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
