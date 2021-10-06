# ForBild: Efficient robust image hashing
A library and command line tool implementation of ForBild written in Rust :crab:. ForBild stands for **For**ensische **Bild**erkennung (german): Forensic image recognition.

**ForBild** is an ***efficient robust image hashing algorithm*** developed at the **Fraunhofer Institute** ad inspried by the **Block Hash** algorithm. It is developed and distributed by Martin Steinebach, Huajian Liu and York Yannikos as a closed source command line tool. The concept of the algorithm is publicly available though.
This implementation follows their approach published in 2012 at the Media watermarking, security and forensics conference. 
http://publica.fraunhofer.de/dokumente/N-206786.html

## Goal: Preventing the distribution of CSAM (Child Sexual Abuse Material) 
For several decades now, image hashes are being used to check CSAM against known images/videos. There are several big databases (e.g. ProjectVic) that use MD5, SHA1 and SHA256 hashes as well as Microsofts robust PhotoDNA hashes (at least some of them). Automatically tagging known images within forensic data can greatly reduce the workload and backlog of investigators :detective:.

## :question: What is robust hashing?
The previously mentioned ***cryptographic hashes*** (e.g. SHA1, SHA256, MD5) can be used to look for known images, but as soon there have been the slightest ***modifications*** (e.g. sending via a messenger), there is a drastic change in the hash itself - rendering it useless. 

***Robust hashes*** or ***perceptual hashes*** on the other hand can cope with these slight to moderate changes. In the fight against the distribution of CSAM, the most widely used robust hash is probably Microsofts **PhotoDNA**. Since it is closed source, there have been attempts to reverse engineer it. While the algorithm itself seems to be quite good, it has several downsides and flaws. There are several open source projects like **pHash** or Facebooks **PDQ-Hash**, but in the eyes of the Fraunhofer researchers nearly all of them are neither efficient nor robust enough. 

## :framed_picture:	ForBild: Yet another hashing algorithm?
The main goal of the researchers was developing a highly efficient robust hashing algorithm that is fast enough for on-line use (e.g. while copying the image itself). In forensic scenarios there usually are millions to billions of images that have to be automatically hashed and checked against. A low computational complexety results in a highly efficient algorithm that can be accepted as an alternative to (or used together with) cryptographic hashes.

### :turtle: Robustness
* **Hash Calculation**: Robust against moderate image modification
  * Application of JPEG compression
  * Changes in brightness, contrast, gamma
  * Changes in color (hue and saturation)
  * Changes in scale and small cropping or mirroring 
  * Addition of smaller text or watermarks
  * Addition of grain or noise (*salt and pepper*)
* **Hash Comparison**: More robust than others 
  * Uses *Hamming distance* first
  * If necessary also uses *quantum hash comparison* 

### :racehorse: Efficiency
* **Hash Calculation**: Computationally and mathematically simple hashing algorithm 
  * No trigonometric functions
  * No deeply nested long for loops
  * No matrix operations
* **Hash Comparison**:
  * Only uses quantum hash comparison when necessary (see below) 

# The Algorithm

## Hash Calculation

### Step 1: Image preprocessing
1. Convert to grayscale
2. Downsample to 16x16 pixels

### Step 2: Image processing
1. Automatic mirroring

### Step 3: Hash calculation
1. Divide into four subareas (each 8x8 pixels)
2. Change pixel value to 0 or 1 depending on the mean value
3. Create binary hash from binary image

## Hash Comparison

### Step 1: Calculating the Hamming distance
1. Bit by bit comparison of binary hashes

### Step 2: Performing a quantum hash comparison
1. NOT WRITTEN YET

### Step 3: Combining both distance metrics
1. NOT WRITTEN YET
