
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
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-H', 'H', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'Specify that the input has no header row')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv')
            [CompletionResult]::new('--tsv', 'tsv', [CompletionResultType]::ParameterName, 'Use ''\t'' as delimiter for tsv')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion file')
            break
        }
        'csview;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
