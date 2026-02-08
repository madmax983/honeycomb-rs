#!/bin/bash
# Remove observability feature flags and just keep the eprintln! statements

for file in src/event.rs src/batch.rs src/client.rs; do
    # Remove #[cfg(feature = "observability")] and following tracing:: lines
    # Keep #[cfg(not(feature = "observability"))] and eprintln! lines
    sed -i '/^.*#\[cfg(feature = "observability")\]/,/^.*tracing::/d' "$file"
    sed -i '/^.*#\[cfg(not(feature = "observability")\)]/d' "$file"
done

echo "Observability feature gates removed"
