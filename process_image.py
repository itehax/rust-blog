#AI generated thumb gen
import os
import time
from pathlib import Path
from PIL import Image
from concurrent.futures import ThreadPoolExecutor

# --- CONFIGURATION ---
INPUT_FOLDER = "blog_images"      
OUTPUT_FOLDER = "./public/images/blog_images"     
TARGET_RATIO = 16 / 11           
TARGET_WIDTH = 800               
# ---------------------

def calculate_crop_box(img_width, img_height, target_ratio):
 
    current_ratio = img_width / img_height
    
    if current_ratio > target_ratio:
        # Image is too wide (Landscape). 
        # Constraint: Height is the limiting factor.
        new_height = img_height
        new_width = int(img_height * target_ratio)
    else:
        # Image is too tall (Portrait/Square).
        # Constraint: Width is the limiting factor.
        new_width = img_width
        new_height = int(img_width / target_ratio)
        
    # Calculate offsets for centering
    left = (img_width - new_width) / 2
    top = (img_height - new_height) / 2
    right = (img_width + new_width) / 2
    bottom = (img_height + new_height) / 2
    
    return (left, top, right, bottom)

def process_single_image(file_path):
    try:
        with Image.open(file_path) as img:
            # Convert RGBA to RGB if necessary (e.g. PNGs)
            if img.mode in ("RGBA", "P"):
                img = img.convert("RGB")
                
            # 1. Geometric Transformation (Crop)
            crop_box = calculate_crop_box(img.width, img.height, TARGET_RATIO)
            img = img.crop(crop_box)
            
            # 2. Resampling (Resize)
            # Calculate height based on ratio to maintain strict aspect
            target_height = int(TARGET_WIDTH / TARGET_RATIO)
            img = img.resize((TARGET_WIDTH, target_height), Image.Resampling.LANCZOS)
            
            # 3. Output Optimization
            output_filename = file_path.stem + ".webp"
            output_path = Path(OUTPUT_FOLDER) / output_filename
            
            # Save as WebP (Superior compression algorithm)
            img.save(output_path, "WEBP", quality=85)
            return f"Processed: {file_path.name}"
            
    except Exception as e:
        return f"Error processing {file_path.name}: {e}"

def main():
    # Setup directories
    Path(OUTPUT_FOLDER).mkdir(exist_ok=True)
    input_path = Path(INPUT_FOLDER)
    
    # Gather all images
    extensions = {'.jpg', '.jpeg', '.png', '.webp'}
    files = [f for f in input_path.iterdir() if f.suffix.lower() in extensions]
    
    if not files:
        print(f"No images found in '{INPUT_FOLDER}'.")
        return

    print(f"Starting parallel processing of {len(files)} images...")
    start_time = time.time()

    with ThreadPoolExecutor() as executor:
        results = list(executor.map(process_single_image, files))
        
    duration = time.time() - start_time
    print(f"\nDone! Processed {len(files)} images in {duration:.2f} seconds.")

if __name__ == "__main__":
    main()