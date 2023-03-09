#!/bin/sh
#SBATCH --job-name=test1
#SBATCH --partition=normal
##SBATCH --output=/scratch/%u/logfiles/%x-%N-%j.out  # Output file`
##SBATCH --error=/scratch/%u/logfiles/%x-%N-%j.err   # Error file`
#SBATCH --mail-type=BEGIN,END,FAIL # ALL,NONE,BEGIN,END,FAIL,REQUEUE,..
#SBATCH --mail-user=ktran44@gmu.edu
#SBATCH --output=/scratch/%u/%x-%N-%A-%a.out
#SBATCH --array=1-12%3

# 3/6/2023 the last job we sent for 500 was p = 0.6
# 3/6/2023 the last jobs we sent for 1000 was p = 0.2, 0.4, 0.6, 0.8
PROBABILITY=0.9
LENGTH=1000
SHUFFLES=`expr "$LENGTH" \* "$LENGTH" \* "$LENGTH" `

module load gnu10
#module load rust

# create the folder that we are going to place the output in
mkdir -p /scratch/ktran44/bernoulli/results/$LENGTH/$PROBABILITY

cd /scratch/ktran44/bernoulli/faster_polyforms/

./target/release/main --length $LENGTH --export analysis --shuffles $SHUFFLES  --norender --bernoulli $PROBABILITY > /scratch/ktran44/bernoulli/results/$LENGTH/$PROBABILITY/polyform_$(uuidgen).txt
