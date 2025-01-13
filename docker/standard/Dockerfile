# Use Python 3.12 slim as base image
FROM python:3.12-slim

# Set working directory
WORKDIR /app

# Install system dependencies including LaTeX and dos2unix for line ending consistency
RUN apt-get update && apt-get install -y \
    build-essential \
    dos2unix \
    git \
    texlive-latex-base \
    texlive-fonts-recommended \
    texlive-fonts-extra \
    texlive-latex-extra \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements file
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the application code
COPY . .

# Set up environment variables
ENV RESEARCH_DIR=/output/research_dir \
    STATE_SAVES_DIR=/output/state_saves \
    TF_CPP_MIN_LOG_LEVEL=2

# Create directory structure and set permissions
RUN mkdir -p /output/research_dir/src /output/research_dir/tex /output/state_saves && \
    chmod -R 777 /output && \
    ln -sf /output/research_dir /app/research_dir && \
    ln -sf /output/state_saves /app/state_saves

# Set default command
ENTRYPOINT ["python", "ai_lab_repo.py"]

# Default command line arguments that can be overridden
CMD ["--llm-backend", "deepseek-chat", "--research-topic", "YOUR_RESEARCH_IDEA"]