/*
* This header is generated by classdump-dyld 1.0
* on Wednesday, November 15, 2023 at 5:38:41â€¯PM Eastern European Standard Time
* Operating System: Version 17.1 (Build 21B74)
* Image Source: /System/Library/PrivateFrameworks/MediaRemote.framework/MediaRemote
* classdump-dyld is licensed under GPLv3, Copyright Â© 2013-2016 by Elias Limneos.
*/

@class MRContentItemMetadata, NSString, NSArray, MRArtwork, NSDictionary, NSData;

@interface MRContentItem : NSObject {

	MRContentItemMetadata* _metadata;
	NSString* _identifier;
	NSString* _info;
	NSArray* _availableLanguageOptions;
	NSArray* _currentLanguageOptions;
	NSArray* _sections;
	NSString* _parentIdentifier;
	NSString* _ancestorIdentifier;
	NSString* _associatedParticipantIdentifier;
	NSString* _queueIdentifier;
	NSString* _requestIdentifier;
	NSArray* _availableArtworkFormats;
	NSArray* _availableRemoteArtworkFormats;
	MRArtwork* _artwork;
	NSDictionary* _artworks;
	NSDictionary* _remoteArtworks;

}

@property (nonatomic,copy) NSString * identifier;                                         //@synthesize identifier=_identifier - In the implementation block
@property (nonatomic,copy) MRContentItemMetadata * metadata; 
@property (nonatomic,copy) NSString * info;                                               //@synthesize info=_info - In the implementation block
@property (nonatomic,copy) NSArray * availableLanguageOptions;                            //@synthesize availableLanguageOptions=_availableLanguageOptions - In the implementation block
@property (nonatomic,copy) NSArray * currentLanguageOptions;                              //@synthesize currentLanguageOptions=_currentLanguageOptions - In the implementation block
@property (nonatomic,copy) NSArray * sections;                                            //@synthesize sections=_sections - In the implementation block
@property (nonatomic,copy) NSString * parentIdentifier;                                   //@synthesize parentIdentifier=_parentIdentifier - In the implementation block
@property (nonatomic,copy) NSString * ancestorIdentifier;                                 //@synthesize ancestorIdentifier=_ancestorIdentifier - In the implementation block
@property (nonatomic,copy) NSString * associatedParticipantIdentifier;                    //@synthesize associatedParticipantIdentifier=_associatedParticipantIdentifier - In the implementation block
@property (nonatomic,copy) NSString * queueIdentifier;                                    //@synthesize queueIdentifier=_queueIdentifier - In the implementation block
@property (nonatomic,copy) NSString * requestIdentifier;                                  //@synthesize requestIdentifier=_requestIdentifier - In the implementation block
@property (nonatomic,retain) MRArtwork * artwork;                                         //@synthesize artwork=_artwork - In the implementation block
@property (nonatomic,retain) NSArray * availableArtworkFormats;                           //@synthesize availableArtworkFormats=_availableArtworkFormats - In the implementation block
@property (nonatomic,retain) NSArray * availableRemoteArtworkFormats;                     //@synthesize availableRemoteArtworkFormats=_availableRemoteArtworkFormats - In the implementation block
@property (nonatomic,retain) NSDictionary * artworks;                                     //@synthesize artworks=_artworks - In the implementation block
@property (nonatomic,retain) NSDictionary * remoteArtworks;                               //@synthesize remoteArtworks=_remoteArtworks - In the implementation block
@property (nonatomic,readonly) MRContentItem * skeleton; 
@property (nonatomic,copy,readonly) NSData * data; 
@property (nonatomic,copy,readonly) NSDictionary * dictionaryRepresentation; 
@property (nonatomic,copy,readonly) NSDictionary * nowPlayingInfo; 
+(id)mergeContentItems:(id)arg1 ;
+(id)extractedIdentifierFromNowPlayingInfo:(id)arg1 ;
-(NSDictionary *)nowPlayingInfo;
-(MRArtwork *)artwork;
-(void)setInfo:(NSString *)arg1 ;
-(void)setArtwork:(MRArtwork *)arg1 ;
-(void)setSections:(NSArray *)arg1 ;
-(MRContentItem *)skeleton;
-(void)setParentIdentifier:(NSString *)arg1 ;
-(id)initWithProtobuf:(id)arg1 ;
-(NSString *)queueIdentifier;
-(NSDictionary *)remoteArtworks;
-(id)initWithIdentifier:(id)arg1 ;
-(void)setRequestIdentifier:(NSString *)arg1 ;
-(NSDictionary *)dictionaryRepresentation;
-(void)setCurrentLanguageOptions:(NSArray *)arg1 ;
-(void)setAssociatedParticipantIdentifier:(NSString *)arg1 ;
-(NSString *)info;
-(NSArray *)sections;
-(id)protobufWithEncoding:(long long)arg1 ;
-(BOOL)isEffectivelyEqual:(id)arg1 ;
-(void)setQueueIdentifier:(NSString *)arg1 ;
-(NSString *)associatedParticipantIdentifier;
-(NSArray *)availableLanguageOptions;
-(BOOL)isEqual:(id)arg1 ;
-(void)mergeFrom:(id)arg1 ;
-(NSArray *)currentLanguageOptions;
-(NSString *)requestIdentifier;
-(void)setNowPlayingInfo:(id)arg1 policy:(unsigned char)arg2 request:(id)arg3 ;
-(NSString *)parentIdentifier;
-(NSString *)identifier;
-(MRContentItemMetadata *)metadata;
-(void)setAvailableRemoteArtworkFormats:(NSArray *)arg1 ;
-(NSArray *)availableArtworkFormats;
-(id)initWithNowPlayingInfo:(id)arg1 ;
-(void)setRemoteArtworks:(NSDictionary *)arg1 ;
-(void)setAvailableArtworkFormats:(NSArray *)arg1 ;
-(NSString *)ancestorIdentifier;
-(void)setAncestorIdentifier:(NSString *)arg1 ;
-(void)setAvailableLanguageOptions:(NSArray *)arg1 ;
-(NSArray *)availableRemoteArtworkFormats;
-(void)setArtworks:(NSDictionary *)arg1 ;
-(void)setMetadata:(MRContentItemMetadata *)arg1 ;
-(id)description;
-(NSData *)data;
-(id)copyWithZone:(NSZone*)arg1 ;
-(id)initWithData:(id)arg1 ;
-(void)setIdentifier:(NSString *)arg1 ;
-(NSDictionary *)artworks;
@end

