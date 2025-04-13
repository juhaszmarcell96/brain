import yaml
from PIL import Image, ImageDraw
import os
import argparse

def load_yaml(file_path):
    with open(file_path, "r") as file:
        return yaml.safe_load(file)

def draw_frame(data, frame_idx, output_dir):
    scene = data["scene"]
    obstacle_cfg = data["obstacle"]
    bird_cfg = data["bird"]
    gameplay = data["gameplay"][frame_idx]
    
    img = Image.new("RGB", (scene["w"], scene["h"]), "white")
    draw = ImageDraw.Draw(img)
    
    # Draw obstacles
    for obs in gameplay["obstacles"]:
        x = obs["x"]
        h_upper = obs["h"]
        h_lower = scene["h"] - h_upper - obstacle_cfg["gap"]
        color = "green"
        if obs["active"]:
            color = "red"
        
        draw.rectangle([x, obstacle_cfg["y"], x + obstacle_cfg["w"], h_upper], fill=color)
        draw.rectangle([x, scene["h"] - h_lower, x + obstacle_cfg["w"], scene["h"]], fill=color)
    
    # Draw birds
    for bird in gameplay["birds"]:
        y = bird["y"]
        draw.rectangle([bird_cfg["x"], y, bird_cfg["x"] + bird_cfg["w"], y + bird_cfg["h"]], fill="red")
    
    # Save frame
    frame_name = f"frame_{frame_idx:06d}.png"
    print(f"saving image to {output_dir}/{frame_name}")
    img.save(os.path.join(output_dir, frame_name))

def generate_frames(yaml_file, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    data = load_yaml(yaml_file)
    
    for i in range(len(data["gameplay"])):
        draw_frame(data, i, output_dir)
    print(f"Frames saved in {output_dir}/")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate frames for Flappy Bird gameplay visualization.")
    parser.add_argument("input_file", type=str, help="Path to the YAML input file.")
    parser.add_argument("output_dir", type=str, help="Path to the output directory where frames will be saved.")
    
    args = parser.parse_args()
    generate_frames(args.input_file, args.output_dir)