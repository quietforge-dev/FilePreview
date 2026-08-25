declare module 'mammoth' {
  interface ConvertToHtmlOptions {
    arrayBuffer: ArrayBuffer;
  }

  interface ConversionResult {
    value: string;
  }

  const mammoth: {
    convertToHtml(options: ConvertToHtmlOptions): Promise<ConversionResult>;
  };

  export default mammoth;
}
