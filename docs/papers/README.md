# Research Papers

This directory contains research papers that form the theoretical foundation of SuperInstance.

## Core Papers

### 1. LoRA: Low-Rank Adaptation of Large Language Models
- **Authors**: Hu, E. J., et al.
- **Link**: [arXiv:2106.09685](https://arxiv.org/abs/2106.09685)
- **Relevance**: Foundation for our LoRA-based breeding system
- **Key Insight**: Low-rank adapters can be composed and merged mathematically

### 2. TIES-Merging: Resolving Interference When Merging Models
- **Authors**: Yadav, P., et al.
- **Link**: [arXiv:2306.01708](https://arxiv.org/abs/2306.01708)
- **Relevance**: Algorithm for merging LoRA weights during breeding
- **Key Insight**: Trim, Elect Sign, and Merge for conflict resolution

### 3. Model Merging by Uncertainty-Based Gradient Matching
- **Authors**: Ilharco, G., et al.
- **Link**: [arXiv:2203.05482](https://arxiv.org/abs/2203.05482)
- **Relevance**: Weight merging strategies for Night School
- **Key Insight**: Uncertainty-based merging improves composed model quality

## Agent Architecture

### 4. ReAct: Synergizing Reasoning and Acting in Language Models
- **Authors**: Yao, S., et al.
- **Link**: [arXiv:2210.03629](https://arxiv.org/abs/2210.03629)
- **Relevance**: Border Collie's intent routing architecture
- **Key Insight**: Interleaving reasoning and action improves task performance

### 5. Toolformer: Language Models Can Teach Themselves to Use Tools
- **Authors**: Schick, T., et al.
- **Link**: [arXiv:2302.02551](https://arxiv.org/abs/2302.02551)
- **Relevance**: Species tool integration
- **Key Insight**: Self-supervised tool learning reduces human annotation

## CRDT & Memory

### 6. CRDTs: Conflict-free Replicated Data Types
- **Authors**: Shapiro, M., et al.
- **Link**: [SSS '11](https://dl.acm.org/doi/10.1145/2050616.2050622)
- **Relevance**: Memory Pasture (smartcrdt integration)
- **Key Insight**: Eventual consistency without coordination

### 7. Automerge: A JSON-like CRDT
- **Authors**: Kleppmann, M., et al.
- **Link**: [PaPoC '22](https://dl.acm.org/doi/10.1145/3517209.3524046)
- **Relevance**: Persistent memory across reboots
- **Key Insight**: JSON-compatible CRDT for application state

## Constraint Solving

### 8. Constraint Handling Rules
- **Authors**: Frühwirth, T.
- **Link**: [Cambridge University Press](https://www.cambridge.org/core/books/constraint-handling-rules/)
- **Relevance**: Geometric routing (constraint-theory integration)
- **Key Insight**: Declarative constraint solving for agent routing

## CUDA Optimization

### 9. CUDA Graphs: Capturing and Launching Workflows
- **Authors**: NVIDIA
- **Link**: [NVIDIA Docs](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#cuda-graphs)
- **Relevance**: Reflex-level <1ms responses (cudaclaw integration)
- **Key Insight**: Kernel graph capture for minimal launch overhead

### 10. TensorRT: High-Performance Inference
- **Authors**: NVIDIA
- **Link**: [NVIDIA TensorRT](https://developer.nvidia.com/tensorrt)
- **Relevance**: Jetson optimization
- **Key Insight**: INT8/FP16 quantization for edge deployment

## Distillation & Evolution

### 11. Distilling the Knowledge in a Neural Network
- **Authors**: Hinton, G., et al.
- **Link**: [arXiv:1503.02531](https://arxiv.org/abs/1503.02531)
- **Relevance**: Night School cloud distillation
- **Key Insight**: Soft targets transfer knowledge efficiently

### 12. Evolution Strategies as a Scalable Alternative to RL
- **Authors**: Salimans, T., et al.
- **Link**: [arXiv:1703.03864](https://arxiv.org/abs/1703.03864)
- **Relevance**: Agent fitness evaluation and breeding
- **Key Insight**: Evolution strategies scale well with parallelization

## Citation

If you use SuperInstance in your research, please cite:

```bibtex
@software{superinstance2024,
  title = {SuperInstance: A Self-Evolving AI Ranch on Edge Hardware},
  author = {SuperInstance Contributors},
  year = {2024},
  url = {https://github.com/SuperInstance/superinstance}
}
```
