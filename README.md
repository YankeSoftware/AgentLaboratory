# Agent Laboratory: Using LLM Agents as Research Assistants


<p align="center">
  <img src="media/AgentLabLogo.png" alt="Demonstration of the flow of AgentClinic" style="width: 99%;">
</p>

<p align="center">
    【English | <a href="readme/README-chinese.md">中文</a> | <a href="readme/README-japanese.md">日本語</a> | <a href="readme/README-korean.md">한국어</a> | <a href="readme/README-filipino.md">Filipino</a> | <a href="readme/README-french.md">Français</a> | <a href="readme/README-slovak.md">Slovenčina</a> | <a href="readme/README-portugese.md">Português</a> | <a href="readme/README-spanish.md">Español</a> | <a href="readme/README-turkish.md">Türkçe</a> | <a href="readme/README-hindi.md">हिंदी</a> | <a href="readme/README-bengali.md">বাংলা</a> | <a href="readme/README-vietnamese.md">Tiếng Việt</a> | <a href="readme/README-russian.md">Русский</a> | <a href="readme/README-arabic.md">العربية</a> | <a href="readme/README-farsi.md">فارسی</a> | <a href="readme/README-italian.md">Italiano</a>】
</p>

<p align="center">
    【📝 <a href="https://arxiv.org/pdf/2501.04227">Paper</a> | 🌐 <a href="https://agentlaboratory.github.io/">Website</a> | 💻 <a href="https://github.com/SamuelSchmidgall/AgentLaboratory">Software</a> | 📰 <a href="https://agentlaboratory.github.io/#citation-ref">Citation</a>】
</p>

## 📖 Overview

- **Agent Laboratory** is an end-to-end autonomous research workflow meant to assist **you** as the human researcher toward **implementing your research ideas**. Agent Laboratory consists of specialized agents driven by large language models to support you through the entire research workflow—from conducting literature reviews and formulating plans to executing experiments and writing comprehensive reports. 
- This system is not designed to replace your creativity but to complement it, enabling you to focus on ideation and critical thinking while automating repetitive and time-intensive tasks like coding and documentation. By accommodating varying levels of computational resources and human involvement, Agent Laboratory aims to accelerate scientific discovery and optimize your research productivity.

<p align="center">
  <img src="media/AgentLab.png" alt="Demonstration of the flow of AgentClinic" style="width: 99%;">
</p>

### 🔬 How does Agent Laboratory work?

- Agent Laboratory consists of three primary phases that systematically guide the research process: (1) Literature Review, (2) Experimentation, and (3) Report Writing. During each phase, specialized agents driven by LLMs collaborate to accomplish distinct objectives, integrating external tools like arXiv, Hugging Face, Python, and LaTeX to optimize outcomes. This structured workflow begins with the independent collection and analysis of relevant research papers, progresses through collaborative planning and data preparation, and results in automated experimentation and comprehensive report generation. Details on specific agent roles and their contributions across these phases are discussed in the paper.

<p align="center">
  <img src="media/AgentLabWF.png" alt="Demonstration of the flow of AgentClinic" style="width: 99%;">
</p>


### 👾 Currently supported models

* **OpenAI**: o1, o1-preview, o1-mini, gpt-4o
* **DeepSeek**: deepseek-chat (deepseek-v3)

To select a specific llm set the flag `--llm-backend="llm_model"` for example `--llm-backend="gpt-4o"` or `--llm-backend="deepseek-chat"`. Please feel free to add a PR supporting new models according to your need!

## 🖥️ Installation

### Quick Start (Docker)

We provide an easy setup for both Windows and Linux users:

1. **Clone the repository**
```bash
git clone git@github.com:SamuelSchmidgall/AgentLaboratory.git
cd AgentLaboratory
```

2. **Choose your platform**:

For Windows Users:
```cmd
:: Run the setup script
run_windows.bat
```

For Linux Users:
```bash
# Run the setup script
./run_linux.sh
```

These scripts will:
- Create necessary directories
- Build the appropriate Docker image
- Set up GPU support if available
- Handle all environment variables and paths

For detailed setup instructions and customization options, see the [Docker Setup Guide](docker/README.md).

### Docker Setup Details

The Docker setup provides a consistent environment across different machines and eliminates the need to install Python and dependencies directly on your host system. We provide three Docker configurations:

1. **Standard Setup** (`Dockerfile`)
   - Full feature set including LaTeX support
   - GPU acceleration enabled
   - All ML libraries included
   - Build time: ~20-30 minutes

2. **Minimal Setup** (`Dockerfile.minimal`)
   - Basic features without LaTeX
   - Faster build time (~5-10 minutes)
   - Smaller image size
   - Perfect for quick experiments

3. **GPU-Optimized** (`Dockerfile.gpu`)
   - CUDA optimization
   - cuDNN and cuBLAS properly configured
   - Best for machine learning tasks
   - Full GPU acceleration support

Choose the setup that best matches your needs:
```bash
# For Windows users:
run_windows.bat [standard|minimal|gpu]

# For Linux users:
./run_linux.sh [standard|minimal|gpu]
```

#### Docker Setup Features

1. **Output Organization**:
   - All outputs are mounted to your local `output` directory
   - `output/research_dir/` contains:
     * `src/` - Generated Python code files
     * `tex/` - LaTeX files
     * `readme.md` - Project documentation
     * `report.txt` - Research report
   - `output/state_saves/` contains checkpoint files for resuming work

2. **Available Options**:
   - `--llm-backend`: Choose between "deepseek-chat" or OpenAI models
   - `--compile-latex`: Set to "false" to disable LaTeX compilation
   - `--copilot-mode`: Set to "true" to enable copilot mode
   - `--load-existing`: Set to "true" to load from a previous state
   - `--load-existing-path`: Specify the state file to load from

#### Example Commands

1. **Using DeepSeek with LaTeX disabled**:
```bash
docker run -it --rm -v "$(pwd)/output:/output" -e DEEPSEEK_API_KEY="your_key" agent-laboratory --llm-backend "deepseek-chat" --compile-latex "false" --research-topic "Topic"
```

2. **Using OpenAI with copilot mode**:
```bash
docker run -it --rm -v "$(pwd)/output:/output" -e OPENAI_API_KEY="your_key" agent-laboratory --llm-backend "o1-mini" --copilot-mode "true" --research-topic "Topic"
```

3. **Loading from previous state**:
```bash
docker run -it --rm -v "$(pwd)/output:/output" -e OPENAI_API_KEY="your_key" agent-laboratory --load-existing "true" --load-existing-path "state_saves/results_interpretation.pkl" --research-topic "Topic"
```

#### Troubleshooting

1. **Build Issues**:
   - If the build seems stuck on LaTeX packages, use the minimal build: `docker build -f Dockerfile.minimal -t agent-laboratory .`
   - For memory issues during build, try increasing Docker's memory limit in Docker Desktop settings

2. **Runtime Issues**:
   - TensorFlow CUDA warnings can be safely ignored (they're suppressed in our setup)
   - Directory errors usually mean the output directory needs proper permissions
   - If you see "directory exists" errors, the container might not have write permissions to your output directory

3. **Quick Solutions**:
   ```cmd
   :: 1. Clean up any previous containers
   docker system prune -f

   :: 2. Create output directory with proper permissions
   mkdir output
   
   :: 3. Use minimal build without LaTeX
   docker build -f Dockerfile.minimal -t agent-laboratory .
   
   :: 4. Run with compile-latex disabled
   docker run -it --rm -v "%CD%/output:/output" -e OPENAI_API_KEY="your_key" -e DEEPSEEK_API_KEY="your_key" agent-laboratory --llm-backend "deepseek-chat" --compile-latex "false" --research-topic "Your topic"
   ```

#### Important Notes

1. **Directory Mounting**:
   - The `-v` flag mounts your local `output` directory to the container
   - All generated files will be accessible in this directory
   - Make sure the directory exists before running the container

2. **API Keys**:
   - Both OpenAI and DeepSeek keys can be used simultaneously
   - Keys are passed through environment variables for security:
     * `OPENAI_API_KEY` for OpenAI models
     * `DEEPSEEK_API_KEY` for DeepSeek models
   - Keys are automatically picked up from environment variables, no need to pass them as command line arguments

3. **Model Selection**:
   - DeepSeek: Use `--llm-backend "deepseek-chat"`
   - OpenAI: Use `--llm-backend "o1-mini"` or other available OpenAI models

4. **State Management**:
   - States are automatically saved in `output/state_saves/`
   - Use `--load-existing` and `--load-existing-path` to resume from a saved state
   - States are saved after each phase for recovery

### Python venv option (Alternative)

* We recommend using python 3.12

1. **Clone the GitHub Repository**: Begin by cloning the repository using the command:
```bash
git clone git@github.com:SamuelSchmidgall/AgentLaboratory.git
```

2. **Set up and Activate Python Environment**
```bash
python -m venv venv_agent_lab
```
- Now activate this environment:
```bash
source venv_agent_lab/bin/activate
```

```
venv_agent_lab\Scripts\activate.bat
```

3. **Install required libraries**
```bash
pip install -r requirements.txt
```

4. **Install pdflatex [OPTIONAL]**
```bash
sudo apt install pdflatex
```
- This enables latex source to be compiled by the agents.
- **[IMPORTANT]** If this step cannot be run due to not having sudo access, pdf compiling can be turned off via running Agent Laboratory via setting the `--compile-latex` flag to false: `--compile-latex "false"`



5. **Now run Agent Laboratory!**

`python ai_lab_repo.py --api-key "API_KEY_HERE" --llm-backend "o1-mini" --research-topic "YOUR RESEARCH IDEA"`

or, if you don't have pdflatex installed

`python ai_lab_repo.py --api-key "API_KEY_HERE" --llm-backend "o1-mini" --research-topic "YOUR RESEARCH IDEA" --compile-latex "false"`

### Co-Pilot mode

To run Agent Laboratory in copilot mode, simply set the copilot-mode flag to `"true"`

`python ai_lab_repo.py --api-key "API_KEY_HERE" --llm-backend "o1-mini" --research-topic "YOUR RESEARCH IDEA" --copilot-mode "true"`

-----
## Tips for better research outcomes


#### [Tip #1] 📝 Make sure to write extensive notes! 📝

**Writing extensive notes is important** for helping your agent understand what you're looking to accomplish in your project, as well as any style preferences. Notes can include any experiments you want the agents to perform, providing API keys, certain plots or figures you want included, or anything you want the agent to know when performing research.

This is also your opportunity to let the agent know **what compute resources it has access to**, e.g. GPUs (how many, what type of GPU, how many GBs), CPUs (how many cores, what type of CPUs), storage limitations, and hardware specs.

In order to add notes, you must modify the task_notes_LLM structure inside of `ai_lab_repo.py`. Provided below is an example set of notes used for some of our experiments. 


```
task_notes_LLM = [
    {"phases": ["plan formulation"],
     "note": f"You should come up with a plan for TWO experiments."},

    {"phases": ["plan formulation", "data preparation",  "running experiments"],
     "note": "Please use gpt-4o-mini for your experiments."},

    {"phases": ["running experiments"],
     "note": f'Use the following code to inference gpt-4o-mini: \nfrom openai import OpenAI\nos.environ["OPENAI_API_KEY"] = "{api_key}"\nclient = OpenAI()\ncompletion = client.chat.completions.create(\nmodel="gpt-4o-mini-2024-07-18", messages=messages)\nanswer = completion.choices[0].message.content\n'},

    {"phases": ["running experiments"],
     "note": f"You have access to only gpt-4o-mini using the OpenAI API, please use the following key {api_key} but do not use too many inferences. Do not use openai.ChatCompletion.create or any openai==0.28 commands. Instead use the provided inference code."},

    {"phases": ["running experiments"],
     "note": "I would recommend using a small dataset (approximately only 100 data points) to run experiments in order to save time. Do not use much more than this unless you have to or are running the final tests."},

    {"phases": ["data preparation", "running experiments"],
     "note": "You are running on a MacBook laptop. You can use 'mps' with PyTorch"},

    {"phases": ["data preparation", "running experiments"],
     "note": "Generate figures with very colorful and artistic design."},
    ]
```

--------

#### [Tip #2] 🚀 Using more powerful models generally leads to better research 🚀

When conducting research, **the choice of model can significantly impact the quality of results**. More powerful models tend to have higher accuracy, better reasoning capabilities, and better report generation. If computational resources allow, prioritize the use of advanced models such as o1-(mini/preview) or similar state-of-the-art large language models.

However, **it’s important to balance performance and cost-effectiveness**. While powerful models may yield better results, they are often more expensive and time-consuming to run. Consider using them selectively—for instance, for key experiments or final analyses—while relying on smaller, more efficient models for iterative tasks or initial prototyping.

When resources are limited, **optimize by fine-tuning smaller models** on your specific dataset or combining pre-trained models with task-specific prompts to achieve the desired balance between performance and computational efficiency.

-----

#### [Tip #3] ✅ You can load previous saves from checkpoints ✅

**If you lose progress, internet connection, or if a subtask fails, you can always load from a previous state.** All of your progress is saved by default in the `state_saves` variable, which stores each individual checkpoint. Just pass the following arguments when running `ai_lab_repo.py`

`python ai_lab_repo.py --api-key "API_KEY_HERE" --research-topic "YOUR RESEARCH IDEA" --llm-backend "o1-mini" --load-existing True --load-existing-path "save_states/LOAD_PATH"`

-----



#### [Tip #4] 🈯 If you are running in a language other than English 🈲

If you are running Agent Laboratory in a language other than English, no problem, just make sure to provide a language flag to the agents to perform research in your preferred language. Note that we have not extensively studied running Agent Laboratory in other languages, so be sure to report any problems you encounter.

For example, if you are running in Chinese:

`python ai_lab_repo.py --api-key "API_KEY_HERE" --research-topic "YOUR RESEARCH IDEA (in your language)" --llm-backend "o1-mini" --language "中文"`

----


#### [Tip #5] 🌟 There is a lot of room for improvement 🌟

There is a lot of room to improve this codebase, so if you end up making changes and want to help the community, please feel free to share the changes you've made! We hope this tool helps you!


## 📜 License

Source Code Licensing: Our project's source code is licensed under the MIT License. This license permits the use, modification, and distribution of the code, subject to certain conditions outlined in the MIT License.

## 📬 Contact

If you would like to get in touch, feel free to reach out to [sschmi46@jhu.edu](mailto:sschmi46@jhu.edu)

## Reference / Bibtex



```bibtex
@misc{schmidgall2025agentlaboratoryusingllm,
      title={Agent Laboratory: Using LLM Agents as Research Assistants}, 
      author={Samuel Schmidgall and Yusheng Su and Ze Wang and Ximeng Sun and Jialian Wu and Xiaodong Yu and Jiang Liu and Zicheng Liu and Emad Barsoum},
      year={2025},
      eprint={2501.04227},
      archivePrefix={arXiv},
      primaryClass={cs.HC},
      url={https://arxiv.org/abs/2501.04227}, 
}
```
