# git-translated-commit

translate Japanese into English with OPENAI_API_KEY and make a commit with the translated message.

## Run

```sh
OPENAI_API_KEY=XXX cargo run <YOUR_MESSAGE_IN_JAPANESE>

# example
OPENAI_API_KEY=XXX cargo run hoge画面のレイアウトを修正しました

# new commit with message "I corrected the layout of the "hoge" screen." is created.
```
