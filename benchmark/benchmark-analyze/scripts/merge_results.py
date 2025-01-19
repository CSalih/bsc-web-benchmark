import os
import json
import matplotlib.pyplot as plt

# Define the directory containing the results
results_dir = '../benchmark-playwright/test-run'

def load_framework_paths(path: str):
    framework_paths = []
    for root, dirs, files in os.walk(path):
        for dir_name in dirs:
            foo = dir_name.split('-')
            if len(foo) != 3:
                continue
            [framework, os_name, timestamp] = foo
            framework_paths.append({
                'framework': framework,
                'os_name': os_name,
                'timestamp': timestamp,
                'path': os.path.join(root, dir_name),
                'metric': {}
            })
    return framework_paths

# Function to load JSON results
def append_metrics(frameworks: list[dict]):
  for framework in frameworks:
    directory = framework.get('path')
    for filename in os.listdir(directory):
      if not filename.endswith('.txt'):
        continue

      [metric, browser] = filename.replace('.txt', '').split('-')

      with open(os.path.join(directory, filename), 'r') as file:
        metrics = file.read().split('\n')
        metrics = [float(x) for x in metrics if x != '']
        framework['metric'].update({
          browser: {
            metric: metrics
          }
        })
  return frameworks


frameworks = load_framework_paths(results_dir)
frameworks = append_metrics(frameworks)

print(json.dumps(frameworks[0], indent=2))
exit(0)

metrics = [framework['metrics'] for framework in frameworks]
labels = [framework['framework'] for framework in frameworks]

# Create a boxplot
plt.boxplot(metrics, labels=labels)
plt.title('Benchmark Response Times')
plt.xlabel('Benchmarks')
plt.ylabel('Response Time (ms)')
plt.show()
