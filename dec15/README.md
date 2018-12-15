For part 2, run

```bash
for power in {4..30}; do
  echo "Power $power and $(./target/release/dec15 input.txt ${power})"
done
```
