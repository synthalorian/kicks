# AI Tone Assistant

The AI Tone Assistant lets you describe a guitar or bass tone in plain English,
and Kicks generates the signal chain parameters to match.

## Supported Providers

Kicks supports two API families:

### Anthropic (Claude)

- Default provider
- Uses the Messages API directly
- Requires an Anthropic API key

### OpenAI-compatible

Works with any API that exposes the OpenAI chat completions format:

| Service | Endpoint URL |
|---------|-------------|
| **OpenAI** | `https://api.openai.com/v1/chat/completions` |
| **OpenRouter** | `https://openrouter.ai/api/v1/chat/completions` |
| **Ollama** (local) | `http://localhost:11434/v1/chat/completions` |
| **llama-server** | `http://localhost:8080/v1/chat/completions` |
| **Groq** | `https://api.groq.com/openai/v1/chat/completions` |
| **Together** | `https://api.together.xyz/v1/chat/completions` |
| **GLM** | `https://open.bigmodel.cn/api/paas/v4/chat/completions` |
| **Kimi** | `https://api.moonshot.cn/v1/chat/completions` |

## Configuration

Go to **Settings → AI Tone Assistant** and set:

1. **Provider** — Anthropic or OpenAI-compatible
2. **Endpoint URL** — defaults to the provider's standard URL
3. **API Key** — your API key (leave empty for local models like Ollama)
4. **Model** — model name (e.g. `claude-sonnet-4-20250514`, `gpt-4o`, `gemini-2.0-flash`, etc.)

## Generating a Tone

1. Go to **AI Assistant** page
2. Describe the tone you want in the prompt field

### Example prompts

- *"Modern metal chug — tight low end, aggressive mids, smooth top end"*
- *"Warm jazz clean — round lows, scooped mids, sparkly highs"*
- *"Edge of breakup blues — touch sensitive, slight grit when digging in"*
- *"Thick bass tone — deep low end, punchy mids, no high end fizz"*
- *"Ambient post-rock — lots of reverb and delay, dreamy cleans"*

3. Click **Generate**
4. Review the generated parameters
5. Click **Apply** to set them on your current signal chain

## How It Works

The AI receives the current signal chain structure and available parameter ranges,
and returns a complete parameter set for each plugin. The assistant is designed
to produce musical, usable settings rather than extreme values.

## Tips

- Be specific about genre and desired character
- Mention "bass" in your prompt if you want BassAmp mode
- Start with a description and tweak from there
- For local models (Ollama), use a model capable of following structured output instructions
