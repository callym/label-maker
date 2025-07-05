import axios from 'axios';

const baseURL = 'http://localhost:3000';
const client = axios.create({
  baseURL,
});

export class Image {
  public static get preview(): string {
    return `${baseURL}/preview`;
  }

  public file_name: string;
  public id: string;

  public dimensions: {
    width: number;
    height: number;
  };

  public original_dimensions: {
    width: number;
    height: number;
  };

  public length_mm: number;

  public threshold: number;
  public inverted: boolean;

  public get url(): string {
    return `${baseURL}/images/${this.id}`;
  }

  private constructor(res: any) {
    this.file_name = res.file_name;
    this.id = res.id;

    this.dimensions = {
      width: res.width,
      height: res.height,
    };

    this.original_dimensions = {
      width: res.original_width,
      height: res.original_height,
    };

    this.length_mm = res.length_mm;

    this.threshold = res.threshold;
    this.inverted = res.inverted;
  }

  static async get_images(): Promise<Image[]> {
    const res = await client.get(`/images`);

    const images = [];
    for (const data of res.data) {
      images.push(new Image(data));
    }

    return images;
  }

  static async upload(file: File) {
    const form = new FormData();
    form.append('file', file);

    const res = await client.post(`/images`, form);

    return new Image(res.data);
  }

  async delete() {
    await client.delete(this.url);
  }

  async invert() {
    this.inverted = !this.inverted;
    await client.post(`${this.url}/invert`, { invert: this.inverted });
  }

  async set_threshold(threshold: number) {
    this.threshold = threshold;
    await client.post(`${this.url}/threshold`, { threshold: this.threshold });
  }
}

export class Printer {
  public ty: string;
  public dpi: number;
  public max_pixels: number;
  public media_type: string;
  public media_width: string;
  public tape_color: string;
  public text_color: string;

  private constructor(res: any) {
    this.ty = res.ty;
    this.dpi = res.dpi;
    this.max_pixels = res.max_pixels;
    this.media_type = res.media_type;
    this.media_width = res.media_width;
    this.tape_color = res.tape_color;
    this.text_color = res.text_color;
  }

  static async get() {
    const res = await client.get('/printer');

    return new Printer(res.data);
  }

  static async refresh() {
    const res = await client.get('printer/refresh');

    return new Printer(res.data);
  }

  static async print(): Promise<void> {
    await client.post('/print');
  }
}
